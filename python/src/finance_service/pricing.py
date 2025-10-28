"""
Option pricing service using Rust backend
"""

from enum import Enum
from typing import Dict, Optional
from dataclasses import dataclass

import pyfinance


class OptionType(Enum):
    """Type of option"""
    CALL = "call"
    PUT = "put"


@dataclass
class OptionParams:
    """Parameters for option pricing"""
    spot_price: float
    strike_price: float
    time_to_expiry: float  # in years
    risk_free_rate: float  # annualized
    volatility: float  # annualized
    dividend_yield: float = 0.0

    def validate(self) -> None:
        """Validate option parameters"""
        if self.spot_price <= 0:
            raise ValueError("Spot price must be positive")
        if self.strike_price <= 0:
            raise ValueError("Strike price must be positive")
        if self.time_to_expiry < 0:
            raise ValueError("Time to expiry cannot be negative")
        if self.volatility < 0:
            raise ValueError("Volatility cannot be negative")


@dataclass
class PricingResult:
    """Result of option pricing calculation"""
    price: float
    delta: float
    gamma: float
    theta: float
    vega: float
    rho: float

    def to_dict(self) -> Dict[str, float]:
        """Convert result to dictionary"""
        return {
            "price": self.price,
            "delta": self.delta,
            "gamma": self.gamma,
            "theta": self.theta,
            "vega": self.vega,
            "rho": self.rho,
        }


class OptionPricer:
    """
    High-performance option pricing service using Black-Scholes model.

    This class provides a Python API for option pricing calculations
    powered by Rust implementations.

    Example:
        >>> pricer = OptionPricer()
        >>> result = pricer.price_option(
        ...     spot_price=100.0,
        ...     strike_price=105.0,
        ...     time_to_expiry=1.0,
        ...     risk_free_rate=0.05,
        ...     volatility=0.2,
        ...     option_type=OptionType.CALL
        ... )
        >>> print(f"Option price: {result.price:.2f}")
        >>> print(f"Delta: {result.delta:.4f}")
    """

    @staticmethod
    def price_option(
        spot_price: float,
        strike_price: float,
        time_to_expiry: float,
        risk_free_rate: float,
        volatility: float,
        option_type: OptionType,
        dividend_yield: float = 0.0,
    ) -> PricingResult:
        """
        Calculate option price and Greeks using Black-Scholes model.

        Args:
            spot_price: Current price of the underlying asset
            strike_price: Strike price of the option
            time_to_expiry: Time to expiry in years
            risk_free_rate: Risk-free interest rate (annualized)
            volatility: Volatility of the underlying asset (annualized)
            option_type: Type of option (CALL or PUT)
            dividend_yield: Dividend yield (annualized), default 0.0

        Returns:
            PricingResult containing price and all Greeks

        Raises:
            ValueError: If any parameter is invalid

        Example:
            >>> pricer = OptionPricer()
            >>> result = pricer.price_option(
            ...     spot_price=100.0,
            ...     strike_price=100.0,
            ...     time_to_expiry=1.0,
            ...     risk_free_rate=0.05,
            ...     volatility=0.2,
            ...     option_type=OptionType.CALL
            ... )
            >>> assert result.price > 0
        """
        # Validate parameters
        params = OptionParams(
            spot_price=spot_price,
            strike_price=strike_price,
            time_to_expiry=time_to_expiry,
            risk_free_rate=risk_free_rate,
            volatility=volatility,
            dividend_yield=dividend_yield,
        )
        params.validate()

        # Call Rust implementation
        result_dict = pyfinance.price_option(
            spot_price=spot_price,
            strike_price=strike_price,
            time_to_expiry=time_to_expiry,
            risk_free_rate=risk_free_rate,
            volatility=volatility,
            dividend_yield=dividend_yield,
            option_type=option_type.value,
        )

        return PricingResult(**result_dict)

    @staticmethod
    def price_call(
        spot_price: float,
        strike_price: float,
        time_to_expiry: float,
        risk_free_rate: float,
        volatility: float,
        dividend_yield: float = 0.0,
    ) -> PricingResult:
        """
        Convenience method to price a call option.

        Args:
            spot_price: Current price of the underlying asset
            strike_price: Strike price of the option
            time_to_expiry: Time to expiry in years
            risk_free_rate: Risk-free interest rate (annualized)
            volatility: Volatility of the underlying asset (annualized)
            dividend_yield: Dividend yield (annualized), default 0.0

        Returns:
            PricingResult containing price and all Greeks
        """
        return OptionPricer.price_option(
            spot_price=spot_price,
            strike_price=strike_price,
            time_to_expiry=time_to_expiry,
            risk_free_rate=risk_free_rate,
            volatility=volatility,
            option_type=OptionType.CALL,
            dividend_yield=dividend_yield,
        )

    @staticmethod
    def price_put(
        spot_price: float,
        strike_price: float,
        time_to_expiry: float,
        risk_free_rate: float,
        volatility: float,
        dividend_yield: float = 0.0,
    ) -> PricingResult:
        """
        Convenience method to price a put option.

        Args:
            spot_price: Current price of the underlying asset
            strike_price: Strike price of the option
            time_to_expiry: Time to expiry in years
            risk_free_rate: Risk-free interest rate (annualized)
            volatility: Volatility of the underlying asset (annualized)
            dividend_yield: Dividend yield (annualized), default 0.0

        Returns:
            PricingResult containing price and all Greeks
        """
        return OptionPricer.price_option(
            spot_price=spot_price,
            strike_price=strike_price,
            time_to_expiry=time_to_expiry,
            risk_free_rate=risk_free_rate,
            volatility=volatility,
            option_type=OptionType.PUT,
            dividend_yield=dividend_yield,
        )
