//! Technical analysis indicators library
//!
//! This library provides implementations of common technical analysis indicators
//! for financial markets, starting with the Exponential Moving Average (EMA).
//!
//! # Example
//!
//! ```
//! use indicator::EMA;
//!
//! // Create EMA with 10-period window
//! let ema = EMA::new(10)?;
//!
//! // Calculate EMA for a series of prices (need at least 10 values for period=10)
//! let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0, 110.0];
//! let result = ema.calculate(&prices)?;
//!
//! println!("EMA values: {:?}", result);
//! # Ok::<(), indicator::IndicatorError>(())
//! ```

use thiserror::Error;

/// Errors that can occur during indicator calculations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum IndicatorError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Insufficient data: {0}")]
    InsufficientData(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Exponential Moving Average (EMA) indicator
///
/// EMA is a type of moving average that places greater weight on recent data points.
/// It responds more quickly to price changes than a simple moving average (SMA).
///
/// # Formula
///
/// EMA(t) = Price(t) × α + EMA(t-1) × (1 - α)
///
/// where α = 2 / (period + 1) is the smoothing factor
///
/// # Example
///
/// ```
/// use indicator::EMA;
///
/// let ema = EMA::new(5)?;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
/// let result = ema.calculate(&prices)?;
///
/// // EMA starts from the first SMA value and applies exponential smoothing
/// assert_eq!(result.len(), prices.len());
/// # Ok::<(), indicator::IndicatorError>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EMA {
    /// Period for the EMA calculation
    period: usize,
    /// Smoothing factor (alpha)
    alpha: f64,
}

impl EMA {
    /// Creates a new EMA indicator with the specified period
    ///
    /// # Arguments
    ///
    /// * `period` - Number of periods for the EMA calculation (must be > 0)
    ///
    /// # Returns
    ///
    /// Returns a configured `EMA` instance or an error if the period is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use indicator::EMA;
    ///
    /// let ema = EMA::new(20)?;
    /// # Ok::<(), indicator::IndicatorError>(())
    /// ```
    pub fn new(period: usize) -> Result<Self, IndicatorError> {
        if period == 0 {
            return Err(IndicatorError::InvalidParameter(
                "Period must be greater than 0".to_string(),
            ));
        }

        // Calculate smoothing factor: α = 2 / (period + 1)
        let alpha = 2.0 / (period as f64 + 1.0);

        Ok(Self { period, alpha })
    }

    /// Calculates EMA for a batch of price data
    ///
    /// The first EMA value is initialized as the simple moving average (SMA)
    /// of the first `period` values. Subsequent values use the exponential formula.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of price data (must have at least `period` values)
    ///
    /// # Returns
    ///
    /// Returns a vector of EMA values with the same length as the input.
    /// The first `period - 1` values will be `None` as there isn't enough data.
    ///
    /// # Example
    ///
    /// ```
    /// use indicator::EMA;
    ///
    /// let ema = EMA::new(3)?;
    /// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
    /// let result = ema.calculate(&prices)?;
    ///
    /// assert_eq!(result.len(), 5);
    /// # Ok::<(), indicator::IndicatorError>(())
    /// ```
    pub fn calculate(&self, prices: &[f64]) -> Result<Vec<Option<f64>>, IndicatorError> {
        if prices.is_empty() {
            return Err(IndicatorError::InsufficientData(
                "Price data cannot be empty".to_string(),
            ));
        }

        if prices.len() < self.period {
            return Err(IndicatorError::InsufficientData(
                format!("Need at least {} data points, got {}", self.period, prices.len()),
            ));
        }

        let mut result = Vec::with_capacity(prices.len());

        // Fill first period-1 values with None
        for _ in 0..self.period - 1 {
            result.push(None);
        }

        // Calculate initial SMA for the first EMA value
        let initial_sma: f64 = prices[..self.period].iter().sum::<f64>() / self.period as f64;
        result.push(Some(initial_sma));

        // Calculate subsequent EMA values
        let mut prev_ema = initial_sma;
        for &price in &prices[self.period..] {
            let ema = self.alpha * price + (1.0 - self.alpha) * prev_ema;
            result.push(Some(ema));
            prev_ema = ema;
        }

        Ok(result)
    }

    /// Updates EMA with a new price value (streaming mode)
    ///
    /// This is useful for real-time calculations where prices arrive one at a time.
    ///
    /// # Arguments
    ///
    /// * `current_ema` - The current EMA value (or None if this is the start)
    /// * `new_price` - The new price to incorporate
    ///
    /// # Returns
    ///
    /// Returns the updated EMA value.
    ///
    /// # Example
    ///
    /// ```
    /// use indicator::EMA;
    ///
    /// let ema = EMA::new(10)?;
    /// let mut current_ema = None;
    ///
    /// // In streaming mode, update EMA as new prices arrive
    /// current_ema = Some(ema.update(current_ema, 100.0));
    /// current_ema = Some(ema.update(current_ema, 102.0));
    /// current_ema = Some(ema.update(current_ema, 101.0));
    /// # Ok::<(), indicator::IndicatorError>(())
    /// ```
    pub fn update(&self, current_ema: Option<f64>, new_price: f64) -> f64 {
        match current_ema {
            Some(ema) => self.alpha * new_price + (1.0 - self.alpha) * ema,
            None => new_price, // If no previous EMA, use the price itself
        }
    }

    /// Returns the period used for EMA calculation
    pub fn period(&self) -> usize {
        self.period
    }

    /// Returns the smoothing factor (alpha) used for EMA calculation
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ema_creation() {
        let ema = EMA::new(10).unwrap();
        assert_eq!(ema.period(), 10);
        assert!((ema.alpha() - 2.0 / 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_ema_invalid_period() {
        let result = EMA::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_ema_calculate() {
        let ema = EMA::new(3).unwrap();
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let result = ema.calculate(&prices).unwrap();

        assert_eq!(result.len(), 5);
        assert!(result[0].is_none());
        assert!(result[1].is_none());
        assert!(result[2].is_some());

        // First EMA value should be SMA of first 3 values
        let expected_first_ema = (10.0 + 11.0 + 12.0) / 3.0;
        assert!((result[2].unwrap() - expected_first_ema).abs() < 1e-10);
    }

    #[test]
    fn test_ema_insufficient_data() {
        let ema = EMA::new(10).unwrap();
        let prices = vec![10.0, 11.0, 12.0];
        let result = ema.calculate(&prices);

        assert!(result.is_err());
        match result {
            Err(IndicatorError::InsufficientData(_)) => {}
            _ => panic!("Expected InsufficientData error"),
        }
    }

    #[test]
    fn test_ema_empty_data() {
        let ema = EMA::new(5).unwrap();
        let prices = vec![];
        let result = ema.calculate(&prices);

        assert!(result.is_err());
    }

    #[test]
    fn test_ema_update_streaming() {
        let ema = EMA::new(3).unwrap();

        // Start with no previous EMA
        let ema1 = ema.update(None, 10.0);
        assert_eq!(ema1, 10.0);

        let ema2 = ema.update(Some(ema1), 12.0);
        // α = 2/(3+1) = 0.5
        // EMA = 0.5 * 12.0 + 0.5 * 10.0 = 11.0
        assert_eq!(ema2, 11.0);

        let ema3 = ema.update(Some(ema2), 14.0);
        // EMA = 0.5 * 14.0 + 0.5 * 11.0 = 12.5
        assert_eq!(ema3, 12.5);
    }

    #[test]
    fn test_ema_monotonic_increasing() {
        let ema = EMA::new(5).unwrap();
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
        let result = ema.calculate(&prices).unwrap();

        // Skip None values and check that EMA values are increasing
        let ema_values: Vec<f64> = result.iter().filter_map(|&x| x).collect();
        for i in 1..ema_values.len() {
            assert!(ema_values[i] > ema_values[i - 1]);
        }
    }

    #[test]
    fn test_ema_responds_to_changes() {
        let ema = EMA::new(3).unwrap();
        // Price spike in the middle
        let prices = vec![100.0, 100.0, 100.0, 150.0, 100.0, 100.0];
        let result = ema.calculate(&prices).unwrap();

        let ema_values: Vec<f64> = result.iter().filter_map(|&x| x).collect();

        // EMA should increase when price spikes
        assert!(ema_values[1] > ema_values[0]);
        // And should start decreasing after the spike
        assert!(ema_values[3] < ema_values[2]);
    }
}
