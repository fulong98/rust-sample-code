"""
Finance Service - High-performance financial calculations

This service provides a Python API for option pricing and technical analysis
indicators, powered by high-performance Rust implementations.
"""

from .pricing import OptionPricer, OptionType
from .indicators import TechnicalIndicators

__version__ = "0.1.0"
__all__ = ["OptionPricer", "OptionType", "TechnicalIndicators"]
