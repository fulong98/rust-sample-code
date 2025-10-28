# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library project that implements:
1. Option pricing calculations
2. Technical analysis indicators (starting with EMA - Exponential Moving Average)
3. Python bindings via PyO3 for use in Python applications

The project exposes high-performance Rust implementations to Python for financial calculations.

## Project Structure

This is a Cargo workspace with three library crates organized under `rust/crates/`:

```
rust-sample-code/
├── Cargo.toml                          # Workspace root
└── rust/
    └── crates/
        ├── pricing/                    # Option pricing library
        │   ├── Cargo.toml
        │   └── src/
        │       └── lib.rs             # Black-Scholes model implementation
        ├── indicator/                  # Technical analysis indicators
        │   ├── Cargo.toml
        │   └── src/
        │       └── lib.rs             # EMA implementation
        └── pyfinance/                  # Python bindings (PyO3)
            ├── Cargo.toml
            └── src/
                └── lib.rs             # Python bindings for pricing & indicator
```

**Pricing Crate (`rust/crates/pricing`):**
- `OptionType` enum (Call/Put)
- `OptionParams` struct for pricing parameters
- `PricingResult` struct with price and Greeks (delta, gamma, theta, vega, rho)
- `BlackScholes::price()` - Black-Scholes-Merton formula implementation
- Dependencies: `statrs` for normal distribution, `thiserror` for error handling

**Indicator Crate (`rust/crates/indicator`):**
- `EMA` struct for Exponential Moving Average calculations
- `calculate()` method for batch processing
- `update()` method for streaming/real-time updates
- Proper validation and error handling for edge cases
- Dependencies: `thiserror` for error handling

**PyFinance Crate (`rust/crates/pyfinance`):**
- Python bindings via PyO3 for both pricing and indicator crates
- `price_option()` function - exposes Black-Scholes pricing to Python
- `EMA` class - exposes EMA indicator to Python with `calculate()` and `update()` methods
- Returns Python dictionaries and native types for easy integration
- Dependencies: `pyo3`, `pricing`, `indicator`

## Development Commands

### Rust Development

**Build the Rust library:**
```bash
cargo build
cargo build --release  # For optimized builds
```

**Run Rust tests:**
```bash
cargo test
cargo test -- --nocapture  # Show println! output
cargo test <test_name>     # Run specific test
```

**Check code without building:**
```bash
cargo check
```

**Format and lint:**
```bash
cargo fmt
cargo clippy
```

### Python + PyO3 Development

**Setup Python environment:**
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

**Build Python module with maturin:**
```bash
maturin develop          # Build and install in current Python env (debug mode)
maturin develop --release  # Build optimized version
```

**Build wheel for distribution:**
```bash
maturin build            # Build wheel in debug mode
maturin build --release  # Build optimized wheel
```

**Run Python examples:**
```bash
# After building with maturin develop
python python/examples/option_pricing_example.py
python python/examples/indicator_example.py
```

**Run Python tests:**
```bash
pytest
python -m pytest tests/
python -m pytest tests/test_file.py::test_function  # Run specific test
```

## Architecture Notes

### Rust Core Library
- Implement numerical/computational logic in pure Rust for performance
- Use established crates for financial calculations where appropriate
- Keep the core logic separate from Python bindings

### PyO3 Bindings Layer
- Create a separate module for PyO3 bindings
- Use `#[pyfunction]` and `#[pyclass]` macros to expose Rust to Python
- Handle type conversions between Rust and Python types
- Add appropriate error handling that translates to Python exceptions

### Python Service Layer
- Located in `python/src/finance_service/`
- Thin wrapper around the Rust library for user-friendly API
- Two main modules:
  - `pricing.py`: OptionPricer class with convenience methods
  - `indicators.py`: TechnicalIndicators class with EMA calculator
- Type hints for all public functions
- Input validation before calling Rust code
- Returns Python-native types (dataclasses, lists, etc.)

## Key Dependencies

**Rust:**
- `pyo3` (v0.22) - Python bindings with cdylib support
- `statrs` (v0.17) - Statistical distributions for pricing
- `thiserror` (v1.0) - Error handling

**Python:**
- `maturin` (>=1.0) - Build tool for PyO3 projects
- Development: `pytest`, `mypy`, `pylint`, `black`, `ruff`

## Testing Strategy

- Write unit tests in Rust for core calculation logic
- Integration tests in Python to verify bindings work correctly
- Test edge cases and numerical accuracy for financial calculations

## Rust Coding Standards

This project follows idiomatic Rust practices based on The Rust Book, Rust API Guidelines, and RFC 430 naming conventions. See instruction.md for complete details.

### Key Principles

**Error Handling:**
- Use `Result<T, E>` for recoverable errors, avoid panics in library code
- Prefer `?` operator over `unwrap()` or `expect()`
- Use `thiserror` or `anyhow` for custom error types
- Provide meaningful error messages with context

**Type Safety:**
- Leverage Rust's ownership system and strong typing
- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary
- Use `&str` instead of `String` for function parameters when you don't need ownership
- Implement common traits where appropriate: `Debug`, `Clone`, `PartialEq`, `Default`

**Performance:**
- Use iterators instead of index-based loops
- Avoid unnecessary allocations, prefer borrowing and zero-copy operations
- Avoid premature `collect()`, keep iterators lazy
- Use `rayon` for CPU-bound parallel tasks

**Code Organization:**
- Split binary and library code (`main.rs` vs `lib.rs`)
- Use modules and public interfaces to encapsulate logic
- Keep `main.rs` or `lib.rs` minimal, move logic to modules
- Private fields in structs for future-proofing

**Documentation:**
- Document all public APIs with rustdoc (`///` comments)
- Include examples that use `?` operator, not `unwrap()`
- Document error conditions and panic scenarios
- Write comprehensive unit tests in `#[cfg(test)]` modules

**Quality Checks:**
- Code must compile without warnings
- Run `cargo fmt` for formatting
- Run `cargo clippy` to catch common mistakes
- Ensure comprehensive test coverage including edge cases
