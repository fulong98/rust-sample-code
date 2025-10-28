//! Financial options pricing library
//!
//! This library provides implementations of option pricing models, starting with the
//! Black-Scholes model for European options.
//!
//! # Example
//!
//! ```
//! use pricing::{OptionType, OptionParams, BlackScholes};
//!
//! let params = OptionParams {
//!     spot_price: 100.0,
//!     strike_price: 100.0,
//!     time_to_expiry: 1.0,      // 1 year
//!     risk_free_rate: 0.05,     // 5%
//!     volatility: 0.2,          // 20%
//!     dividend_yield: 0.0,
//! };
//!
//! let result = BlackScholes::price(&params, OptionType::Call)?;
//! println!("Call option price: {:.2}", result.price);
//! println!("Delta: {:.4}", result.delta);
//! # Ok::<(), pricing::PricingError>(())
//! ```

use statrs::distribution::{ContinuousCDF, Normal};
use thiserror::Error;

/// Errors that can occur during option pricing calculations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum PricingError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Type of option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionType {
    /// Call option - right to buy
    Call,
    /// Put option - right to sell
    Put,
}

/// Parameters for option pricing
#[derive(Debug, Clone, PartialEq)]
pub struct OptionParams {
    /// Current price of the underlying asset
    pub spot_price: f64,
    /// Strike price of the option
    pub strike_price: f64,
    /// Time to expiry in years
    pub time_to_expiry: f64,
    /// Risk-free interest rate (annualized)
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (annualized)
    pub volatility: f64,
    /// Dividend yield (annualized)
    pub dividend_yield: f64,
}

impl OptionParams {
    /// Validates option parameters
    ///
    /// Returns an error if any parameter is invalid (negative values where not allowed,
    /// or values that would make the calculation undefined).
    pub fn validate(&self) -> Result<(), PricingError> {
        if self.spot_price <= 0.0 {
            return Err(PricingError::InvalidParameter(
                "Spot price must be positive".to_string(),
            ));
        }
        if self.strike_price <= 0.0 {
            return Err(PricingError::InvalidParameter(
                "Strike price must be positive".to_string(),
            ));
        }
        if self.time_to_expiry < 0.0 {
            return Err(PricingError::InvalidParameter(
                "Time to expiry cannot be negative".to_string(),
            ));
        }
        if self.volatility < 0.0 {
            return Err(PricingError::InvalidParameter(
                "Volatility cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

/// Result of option pricing calculation including Greeks
#[derive(Debug, Clone, PartialEq)]
pub struct PricingResult {
    /// Option price
    pub price: f64,
    /// Delta - rate of change of option price with respect to underlying price
    pub delta: f64,
    /// Gamma - rate of change of delta with respect to underlying price
    pub gamma: f64,
    /// Theta - rate of change of option price with respect to time
    pub theta: f64,
    /// Vega - rate of change of option price with respect to volatility
    pub vega: f64,
    /// Rho - rate of change of option price with respect to interest rate
    pub rho: f64,
}

/// Black-Scholes option pricing model
///
/// Implements the Black-Scholes-Merton formula for European options.
pub struct BlackScholes;

impl BlackScholes {
    /// Calculates option price and Greeks using the Black-Scholes formula
    ///
    /// # Arguments
    ///
    /// * `params` - Option parameters including spot price, strike, time to expiry, etc.
    /// * `option_type` - Type of option (Call or Put)
    ///
    /// # Returns
    ///
    /// Returns `PricingResult` containing the option price and all Greeks, or a
    /// `PricingError` if the parameters are invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use pricing::{OptionType, OptionParams, BlackScholes};
    ///
    /// let params = OptionParams {
    ///     spot_price: 100.0,
    ///     strike_price: 105.0,
    ///     time_to_expiry: 0.5,
    ///     risk_free_rate: 0.03,
    ///     volatility: 0.25,
    ///     dividend_yield: 0.01,
    /// };
    ///
    /// let result = BlackScholes::price(&params, OptionType::Put)?;
    /// assert!(result.price > 0.0);
    /// # Ok::<(), pricing::PricingError>(())
    /// ```
    pub fn price(params: &OptionParams, option_type: OptionType) -> Result<PricingResult, PricingError> {
        params.validate()?;

        // Handle edge case: at expiry
        if params.time_to_expiry == 0.0 {
            return Self::price_at_expiry(params, option_type);
        }

        let normal = Normal::new(0.0, 1.0)
            .map_err(|e| PricingError::CalculationError(format!("Failed to create normal distribution: {}", e)))?;

        // Calculate d1 and d2
        let sqrt_t = params.time_to_expiry.sqrt();
        let d1 = (
            (params.spot_price / params.strike_price).ln()
                + (params.risk_free_rate - params.dividend_yield + 0.5 * params.volatility.powi(2))
                    * params.time_to_expiry
        ) / (params.volatility * sqrt_t);

        let d2 = d1 - params.volatility * sqrt_t;

        // Calculate price based on option type
        let (price, delta) = match option_type {
            OptionType::Call => {
                let nd1 = normal.cdf(d1);
                let nd2 = normal.cdf(d2);
                let price = params.spot_price * (-params.dividend_yield * params.time_to_expiry).exp() * nd1
                    - params.strike_price * (-params.risk_free_rate * params.time_to_expiry).exp() * nd2;
                let delta = (-params.dividend_yield * params.time_to_expiry).exp() * nd1;
                (price, delta)
            }
            OptionType::Put => {
                let n_neg_d1 = normal.cdf(-d1);
                let n_neg_d2 = normal.cdf(-d2);
                let price = params.strike_price * (-params.risk_free_rate * params.time_to_expiry).exp() * n_neg_d2
                    - params.spot_price * (-params.dividend_yield * params.time_to_expiry).exp() * n_neg_d1;
                let delta = -(-params.dividend_yield * params.time_to_expiry).exp() * n_neg_d1;
                (price, delta)
            }
        };

        // Calculate Greeks
        let gamma = Self::calculate_gamma(params, d1, &normal);
        let theta = Self::calculate_theta(params, d1, d2, option_type, &normal);
        let vega = Self::calculate_vega(params, d1, &normal);
        let rho = Self::calculate_rho(params, d2, option_type, &normal);

        Ok(PricingResult {
            price,
            delta,
            gamma,
            theta,
            vega,
            rho,
        })
    }

    /// Calculates option price at expiry (intrinsic value)
    fn price_at_expiry(params: &OptionParams, option_type: OptionType) -> Result<PricingResult, PricingError> {
        let intrinsic_value = match option_type {
            OptionType::Call => (params.spot_price - params.strike_price).max(0.0),
            OptionType::Put => (params.strike_price - params.spot_price).max(0.0),
        };

        Ok(PricingResult {
            price: intrinsic_value,
            delta: if intrinsic_value > 0.0 {
                match option_type {
                    OptionType::Call => 1.0,
                    OptionType::Put => -1.0,
                }
            } else {
                0.0
            },
            gamma: 0.0,
            theta: 0.0,
            vega: 0.0,
            rho: 0.0,
        })
    }

    fn calculate_gamma(params: &OptionParams, d1: f64, _normal: &Normal) -> f64 {
        let pdf_d1 = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let sqrt_t = params.time_to_expiry.sqrt();

        (-params.dividend_yield * params.time_to_expiry).exp() * pdf_d1
            / (params.spot_price * params.volatility * sqrt_t)
    }

    fn calculate_theta(
        params: &OptionParams,
        d1: f64,
        d2: f64,
        option_type: OptionType,
        normal: &Normal,
    ) -> f64 {
        let pdf_d1 = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let sqrt_t = params.time_to_expiry.sqrt();

        let term1 = -params.spot_price * pdf_d1 * params.volatility
            * (-params.dividend_yield * params.time_to_expiry).exp()
            / (2.0 * sqrt_t);

        match option_type {
            OptionType::Call => {
                let term2 = params.dividend_yield * params.spot_price
                    * normal.cdf(d1)
                    * (-params.dividend_yield * params.time_to_expiry).exp();
                let term3 = params.risk_free_rate * params.strike_price
                    * (-params.risk_free_rate * params.time_to_expiry).exp()
                    * normal.cdf(d2);
                term1 + term2 - term3
            }
            OptionType::Put => {
                let term2 = params.dividend_yield * params.spot_price
                    * normal.cdf(-d1)
                    * (-params.dividend_yield * params.time_to_expiry).exp();
                let term3 = params.risk_free_rate * params.strike_price
                    * (-params.risk_free_rate * params.time_to_expiry).exp()
                    * normal.cdf(-d2);
                term1 - term2 + term3
            }
        }
    }

    fn calculate_vega(params: &OptionParams, d1: f64, _normal: &Normal) -> f64 {
        let pdf_d1 = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let sqrt_t = params.time_to_expiry.sqrt();

        params.spot_price * (-params.dividend_yield * params.time_to_expiry).exp()
            * pdf_d1 * sqrt_t / 100.0  // Divide by 100 to express per 1% change
    }

    fn calculate_rho(
        params: &OptionParams,
        d2: f64,
        option_type: OptionType,
        normal: &Normal,
    ) -> f64 {
        match option_type {
            OptionType::Call => {
                params.strike_price * params.time_to_expiry
                    * (-params.risk_free_rate * params.time_to_expiry).exp()
                    * normal.cdf(d2) / 100.0  // Divide by 100 to express per 1% change
            }
            OptionType::Put => {
                -params.strike_price * params.time_to_expiry
                    * (-params.risk_free_rate * params.time_to_expiry).exp()
                    * normal.cdf(-d2) / 100.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_option_pricing() {
        let params = OptionParams {
            spot_price: 100.0,
            strike_price: 100.0,
            time_to_expiry: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
        };

        let result = BlackScholes::price(&params, OptionType::Call).unwrap();

        // Call option should have positive price
        assert!(result.price > 0.0);
        // Delta for ATM call should be around 0.5-0.6
        assert!(result.delta > 0.4 && result.delta < 0.7);
        // Gamma should be positive
        assert!(result.gamma > 0.0);
    }

    #[test]
    fn test_put_option_pricing() {
        let params = OptionParams {
            spot_price: 100.0,
            strike_price: 100.0,
            time_to_expiry: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
        };

        let result = BlackScholes::price(&params, OptionType::Put).unwrap();

        // Put option should have positive price
        assert!(result.price > 0.0);
        // Delta for ATM put should be around -0.4 to -0.5
        assert!(result.delta < -0.3 && result.delta > -0.6);
        // Gamma should be positive
        assert!(result.gamma > 0.0);
    }

    #[test]
    fn test_invalid_parameters() {
        let params = OptionParams {
            spot_price: -100.0,  // Invalid: negative spot price
            strike_price: 100.0,
            time_to_expiry: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
        };

        let result = BlackScholes::price(&params, OptionType::Call);
        assert!(result.is_err());
    }

    #[test]
    fn test_option_at_expiry() {
        let params = OptionParams {
            spot_price: 110.0,
            strike_price: 100.0,
            time_to_expiry: 0.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
        };

        let call_result = BlackScholes::price(&params, OptionType::Call).unwrap();
        assert!((call_result.price - 10.0).abs() < 1e-10);

        let put_result = BlackScholes::price(&params, OptionType::Put).unwrap();
        assert!((put_result.price - 0.0).abs() < 1e-10);
    }
}
