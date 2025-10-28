# PyFinance - High-Performance Financial Calculations

A high-performance financial calculations library with Rust backend and Python bindings.

## What Does This Project Do?

This project provides **fast financial calculations** by:
1. Writing the heavy math in **Rust** (super fast!)
2. Exposing it to **Python** (easy to use!)

**Features:**
- **Option Pricing**: Black-Scholes model with Greeks (Delta, Gamma, Theta, Vega, Rho)
- **Technical Indicators**: EMA (Exponential Moving Average)

## How It Works (The Flow)

```
┌─────────────────────┐
│  You write Python   │  ← Easy to use!
│  code using our API │
└──────────┬──────────┘
           │
           ↓ Python calls...
┌─────────────────────┐
│  Python Wrapper     │  ← Validates input, nice API
│  (finance_service)  │
└──────────┬──────────┘
           │
           ↓ Calls Rust via PyO3...
┌─────────────────────┐
│  Rust Core Library  │  ← Does the heavy math (FAST!)
│  (pricing/indicator)│
└─────────────────────┘
```

**Example Flow:**
1. You call `OptionPricer.price_call(spot_price=100, ...)` in Python
2. Python wrapper validates your inputs
3. PyO3 sends the data to Rust
4. Rust calculates Black-Scholes formula (super fast!)
5. PyO3 sends the result back to Python
6. You get a nice Python object with the result

## Project Structure

```
rust-sample-code/
├── rust/crates/          # Rust implementation
│   ├── pricing/          # Option pricing library
│   ├── indicator/        # Technical analysis indicators
│   └── pyfinance/        # PyO3 bindings
├── python/               # Python wrapper and examples
│   ├── src/finance_service/  # Python service layer
│   └── examples/         # Usage examples
└── pyproject.toml        # Python package configuration
```

## How to Use It

### Step 1: Install Prerequisites

You need:
- **Rust** (for compiling the fast code)
- **Python 3.8+** (for running your code)

```bash
# Install Rust (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installations
rustc --version
python --version
```

### Step 2: Setup the Project

```bash
# 1. Clone and enter the project
cd rust-sample-code

# 2. Create a Python virtual environment (IMPORTANT!)
python -m venv venv

# 3. Activate it
source venv/bin/activate  # On Windows: venv\Scripts\activate

# 4. Install maturin (the tool that connects Rust and Python)
pip install maturin

# 5. Build the Rust code and install it in Python
maturin develop
```

**Important Note:** If you're using conda, deactivate it first with `conda deactivate` before activating venv!

### Step 3: Set Python Path

```bash
# Add the Python source directory to your PYTHONPATH
export PYTHONPATH=$PYTHONPATH:$(pwd)/python/src
```

### Step 4: Run Examples!

```bash
# Try the option pricing example
python python/examples/option_pricing_example.py

# Try the technical indicator example
python python/examples/indicator_example.py
```

## Code Examples

### Example 1: Price an Option

```python
from finance_service import OptionPricer

# Price a call option
result = OptionPricer.price_call(
    spot_price=100.0,        # Current stock price
    strike_price=105.0,      # Strike price
    time_to_expiry=1.0,      # 1 year until expiration
    risk_free_rate=0.05,     # 5% risk-free rate
    volatility=0.2,          # 20% volatility
)

print(f"Option Price: ${result.price:.2f}")
print(f"Delta: {result.delta:.4f}")
print(f"Gamma: {result.gamma:.4f}")
print(f"Theta: {result.theta:.4f}")
print(f"Vega: {result.vega:.4f}")
print(f"Rho: {result.rho:.4f}")
```

### Example 2: Calculate EMA

```python
from finance_service import TechnicalIndicators

indicators = TechnicalIndicators()

# Calculate EMA for a list of prices
prices = [100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0, 110.0]
ema_values = indicators.ema(prices, period=10)

print(f"Latest EMA: {ema_values[-1]:.2f}")
```

### Example 3: Streaming EMA (Real-time)

```python
from finance_service import TechnicalIndicators

indicators = TechnicalIndicators()

# Create a streaming calculator
calculator = indicators.ema_streaming(period=10)

# Update with new prices as they come in
for price in [100.0, 102.0, 101.0, 103.0]:
    ema = calculator.update(price)
    print(f"Price: {price}, EMA: {ema:.2f}")
```

## Troubleshooting

### "ModuleNotFoundError: No module named 'finance_service'"
- Make sure you ran `export PYTHONPATH=$PYTHONPATH:$(pwd)/python/src`
- Or run examples from the project root directory

### "ModuleNotFoundError: No module named 'pyfinance'"
- You need to build the Rust code first: `maturin develop`

### "Both VIRTUAL_ENV and CONDA_PREFIX are set"
- You have both conda and venv active
- Run `conda deactivate` first, then activate venv

### Build errors
- Make sure Rust is installed: `rustc --version`
- Update Rust: `rustup update`

## For Developers

### Testing the Rust Code

```bash
# Run Rust tests
cargo test

# Check code compiles
cargo check

# Lint and format
cargo clippy
cargo fmt
```

### Rebuilding After Changes

```bash
# After changing Rust code
maturin develop

# For faster builds (but slower runtime)
maturin develop

# For optimized builds (slower build, faster runtime)
maturin develop --release
```

## Why Rust + Python?

**Rust** is used for:
- ✅ Fast mathematical calculations
- ✅ Type safety and memory safety
- ✅ No garbage collection overhead

**Python** is used for:
- ✅ Easy-to-use API
- ✅ Integration with other Python tools
- ✅ Quick prototyping

**Best of both worlds!**

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please ensure:
- Rust code passes `cargo clippy` and `cargo test`
- Python code passes `mypy` and `pylint`
- All examples work correctly
