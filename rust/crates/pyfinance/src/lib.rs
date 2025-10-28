//! Python bindings for pricing and indicator libraries
//!
//! This module exposes Rust implementations of option pricing and technical analysis
//! indicators to Python via PyO3.
//!
//! # Installation
//!
//! Build and install with maturin:
//! ```bash
//! maturin develop  # For development
//! maturin build --release  # For production
//! ```
//!
//! # Usage in Python
//!
//! ```python
//! import pyfinance
//!
//! # Option pricing
//! result = pyfinance.price_option(
//!     spot_price=100.0,
//!     strike_price=105.0,
//!     time_to_expiry=1.0,
//!     risk_free_rate=0.05,
//!     volatility=0.2,
//!     dividend_yield=0.0,
//!     option_type="call"
//! )
//! print(f"Price: {result['price']}, Delta: {result['delta']}")
//!
//! # EMA calculation
//! ema = pyfinance.EMA(period=10)
//! prices = [100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0, 110.0]
//! result = ema.calculate(prices)
//! print(f"EMA values: {result}")
//! ```

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;

/// Python wrapper for option pricing
///
/// # Arguments
///
/// * `spot_price` - Current price of the underlying asset
/// * `strike_price` - Strike price of the option
/// * `time_to_expiry` - Time to expiry in years
/// * `risk_free_rate` - Risk-free interest rate (annualized)
/// * `volatility` - Volatility of the underlying asset (annualized)
/// * `dividend_yield` - Dividend yield (annualized)
/// * `option_type` - Type of option: "call" or "put"
///
/// # Returns
///
/// Dictionary containing:
/// - `price`: Option price
/// - `delta`: Delta Greek
/// - `gamma`: Gamma Greek
/// - `theta`: Theta Greek
/// - `vega`: Vega Greek
/// - `rho`: Rho Greek
#[pyfunction]
#[pyo3(signature = (spot_price, strike_price, time_to_expiry, risk_free_rate, volatility, dividend_yield, option_type))]
fn price_option(
    py: Python,
    spot_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
    dividend_yield: f64,
    option_type: &str,
) -> PyResult<PyObject> {
    // Parse option type
    let opt_type = match option_type.to_lowercase().as_str() {
        "call" => pricing::OptionType::Call,
        "put" => pricing::OptionType::Put,
        _ => return Err(PyValueError::new_err("option_type must be 'call' or 'put'")),
    };

    // Create option parameters
    let params = pricing::OptionParams {
        spot_price,
        strike_price,
        time_to_expiry,
        risk_free_rate,
        volatility,
        dividend_yield,
    };

    // Calculate price
    let result = pricing::BlackScholes::price(&params, opt_type)
        .map_err(|e| PyValueError::new_err(format!("Pricing error: {}", e)))?;

    // Convert to Python dictionary
    let dict = PyDict::new_bound(py);
    dict.set_item("price", result.price)?;
    dict.set_item("delta", result.delta)?;
    dict.set_item("gamma", result.gamma)?;
    dict.set_item("theta", result.theta)?;
    dict.set_item("vega", result.vega)?;
    dict.set_item("rho", result.rho)?;

    Ok(dict.into())
}

/// Python wrapper for EMA (Exponential Moving Average) indicator
#[allow(clippy::upper_case_acronyms)]
#[pyclass]
struct EMA {
    inner: indicator::EMA,
}

#[pymethods]
impl EMA {
    /// Create a new EMA indicator with the specified period
    ///
    /// # Arguments
    ///
    /// * `period` - Number of periods for the EMA calculation
    ///
    /// # Example
    ///
    /// ```python
    /// ema = pyfinance.EMA(period=10)
    /// ```
    #[new]
    fn new(period: usize) -> PyResult<Self> {
        let inner = indicator::EMA::new(period)
            .map_err(|e| PyValueError::new_err(format!("EMA creation error: {}", e)))?;
        Ok(Self { inner })
    }

    /// Calculate EMA for a batch of prices
    ///
    /// # Arguments
    ///
    /// * `prices` - List of price values
    ///
    /// # Returns
    ///
    /// List of EMA values. The first (period-1) values will be None.
    ///
    /// # Example
    ///
    /// ```python
    /// ema = pyfinance.EMA(period=3)
    /// prices = [10.0, 11.0, 12.0, 13.0, 14.0]
    /// result = ema.calculate(prices)
    /// # result = [None, None, 11.0, 12.0, 13.0]
    /// ```
    fn calculate(&self, prices: Vec<f64>) -> PyResult<Vec<Option<f64>>> {
        self.inner
            .calculate(&prices)
            .map_err(|e| PyValueError::new_err(format!("EMA calculation error: {}", e)))
    }

    /// Update EMA with a new price (streaming mode)
    ///
    /// # Arguments
    ///
    /// * `current_ema` - Current EMA value (or None if starting)
    /// * `new_price` - New price to incorporate
    ///
    /// # Returns
    ///
    /// Updated EMA value
    ///
    /// # Example
    ///
    /// ```python
    /// ema = pyfinance.EMA(period=10)
    /// current = None
    /// current = ema.update(current, 100.0)
    /// current = ema.update(current, 102.0)
    /// ```
    #[pyo3(signature = (current_ema, new_price))]
    fn update(&self, current_ema: Option<f64>, new_price: f64) -> f64 {
        self.inner.update(current_ema, new_price)
    }

    /// Get the period used for EMA calculation
    #[pyo3(name = "period")]
    fn get_period(&self) -> usize {
        self.inner.period()
    }

    /// Get the smoothing factor (alpha) used for EMA calculation
    #[pyo3(name = "alpha")]
    fn get_alpha(&self) -> f64 {
        self.inner.alpha()
    }

    /// String representation of the EMA
    fn __repr__(&self) -> String {
        format!("EMA(period={})", self.inner.period())
    }
}

/// Python module for financial calculations
#[pymodule]
fn pyfinance(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(price_option, m)?)?;
    m.add_class::<EMA>()?;
    Ok(())
}
