"""
Technical analysis indicators service using Rust backend
"""

from typing import List, Optional

import pyfinance


class TechnicalIndicators:
    """
    High-performance technical analysis indicators.

    This class provides a Python API for technical analysis indicators
    powered by Rust implementations.

    Example:
        >>> indicators = TechnicalIndicators()
        >>> prices = [100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0, 110.0]
        >>> ema_values = indicators.ema(prices, period=10)
        >>> print(f"Latest EMA: {ema_values[-1]:.2f}")
    """

    @staticmethod
    def ema(prices: List[float], period: int) -> List[Optional[float]]:
        """
        Calculate Exponential Moving Average (EMA) for a series of prices.

        EMA gives more weight to recent prices and responds more quickly to
        price changes than a simple moving average.

        Args:
            prices: List of price values
            period: Number of periods for EMA calculation (must be > 0)

        Returns:
            List of EMA values. The first (period-1) values will be None
            as there isn't enough data to calculate EMA.

        Raises:
            ValueError: If period is invalid or insufficient data

        Example:
            >>> indicators = TechnicalIndicators()
            >>> prices = [10.0, 11.0, 12.0, 13.0, 14.0]
            >>> ema_values = indicators.ema(prices, period=3)
            >>> # First two values are None, then EMA starts
            >>> assert ema_values[0] is None
            >>> assert ema_values[1] is None
            >>> assert ema_values[2] is not None
        """
        if period <= 0:
            raise ValueError("Period must be greater than 0")
        if len(prices) < period:
            raise ValueError(f"Need at least {period} data points, got {len(prices)}")

        ema_calculator = pyfinance.EMA(period=period)
        return ema_calculator.calculate(prices)

    @staticmethod
    def ema_streaming(period: int) -> "EMACalculator":
        """
        Create a streaming EMA calculator for real-time updates.

        This is useful when prices arrive one at a time and you want to
        update the EMA incrementally.

        Args:
            period: Number of periods for EMA calculation

        Returns:
            EMACalculator instance for streaming calculations

        Example:
            >>> indicators = TechnicalIndicators()
            >>> calculator = indicators.ema_streaming(period=10)
            >>> calculator.update(100.0)
            100.0
            >>> calculator.update(102.0)
            101.818...
        """
        return EMACalculator(period)


class EMACalculator:
    """
    Streaming EMA calculator for real-time updates.

    This class maintains state between updates, allowing you to calculate
    EMA values as new prices arrive one at a time.

    Example:
        >>> calculator = EMACalculator(period=5)
        >>> for price in [100, 102, 101, 103, 105]:
        ...     ema = calculator.update(price)
        ...     print(f"Price: {price}, EMA: {ema:.2f}")
    """

    def __init__(self, period: int):
        """
        Initialize EMA calculator.

        Args:
            period: Number of periods for EMA calculation (must be > 0)

        Raises:
            ValueError: If period is invalid
        """
        if period <= 0:
            raise ValueError("Period must be greater than 0")

        self._calculator = pyfinance.EMA(period=period)
        self._current_ema: Optional[float] = None

    def update(self, price: float) -> float:
        """
        Update EMA with a new price.

        Args:
            price: New price value

        Returns:
            Updated EMA value

        Example:
            >>> calculator = EMACalculator(period=3)
            >>> ema1 = calculator.update(10.0)
            >>> ema2 = calculator.update(12.0)
            >>> ema3 = calculator.update(14.0)
            >>> assert ema3 > ema2 > ema1
        """
        self._current_ema = self._calculator.update(self._current_ema, price)
        return self._current_ema

    @property
    def current_value(self) -> Optional[float]:
        """Get the current EMA value"""
        return self._current_ema

    @property
    def period(self) -> int:
        """Get the period used for calculation"""
        return self._calculator.period

    @property
    def alpha(self) -> float:
        """Get the smoothing factor (alpha) used for calculation"""
        return self._calculator.alpha

    def reset(self) -> None:
        """Reset the calculator state"""
        self._current_ema = None
