"""
Example usage of the option pricing service
"""

from finance_service import OptionPricer, OptionType


def main():
    """Demonstrate option pricing functionality"""
    print("=" * 60)
    print("Option Pricing Examples")
    print("=" * 60)

    # Example 1: Price a call option
    print("\n1. Call Option Pricing:")
    print("-" * 60)
    call_result = OptionPricer.price_call(
        spot_price=100.0,
        strike_price=105.0,
        time_to_expiry=1.0,  # 1 year
        risk_free_rate=0.05,  # 5%
        volatility=0.2,  # 20%
    )
    print(f"Spot Price: $100.00")
    print(f"Strike Price: $105.00")
    print(f"Time to Expiry: 1 year")
    print(f"Risk-Free Rate: 5%")
    print(f"Volatility: 20%")
    print(f"\nResults:")
    print(f"  Option Price: ${call_result.price:.2f}")
    print(f"  Delta: {call_result.delta:.4f}")
    print(f"  Gamma: {call_result.gamma:.4f}")
    print(f"  Theta: {call_result.theta:.4f}")
    print(f"  Vega: {call_result.vega:.4f}")
    print(f"  Rho: {call_result.rho:.4f}")

    # Example 2: Price a put option
    print("\n2. Put Option Pricing:")
    print("-" * 60)
    put_result = OptionPricer.price_put(
        spot_price=100.0,
        strike_price=95.0,
        time_to_expiry=0.5,  # 6 months
        risk_free_rate=0.03,  # 3%
        volatility=0.25,  # 25%
    )
    print(f"Spot Price: $100.00")
    print(f"Strike Price: $95.00")
    print(f"Time to Expiry: 0.5 years (6 months)")
    print(f"Risk-Free Rate: 3%")
    print(f"Volatility: 25%")
    print(f"\nResults:")
    print(f"  Option Price: ${put_result.price:.2f}")
    print(f"  Delta: {put_result.delta:.4f}")
    print(f"  Gamma: {put_result.gamma:.4f}")
    print(f"  Theta: {put_result.theta:.4f}")
    print(f"  Vega: {put_result.vega:.4f}")
    print(f"  Rho: {put_result.rho:.4f}")

    # Example 3: At-the-money options
    print("\n3. At-The-Money (ATM) Options:")
    print("-" * 60)
    atm_call = OptionPricer.price_option(
        spot_price=100.0,
        strike_price=100.0,
        time_to_expiry=0.25,  # 3 months
        risk_free_rate=0.04,  # 4%
        volatility=0.3,  # 30%
        option_type=OptionType.CALL,
    )
    atm_put = OptionPricer.price_option(
        spot_price=100.0,
        strike_price=100.0,
        time_to_expiry=0.25,  # 3 months
        risk_free_rate=0.04,  # 4%
        volatility=0.3,  # 30%
        option_type=OptionType.PUT,
    )
    print(f"Spot Price: $100.00 (ATM)")
    print(f"Strike Price: $100.00")
    print(f"Time to Expiry: 0.25 years (3 months)")
    print(f"Risk-Free Rate: 4%")
    print(f"Volatility: 30%")
    print(f"\nCall Option:")
    print(f"  Price: ${atm_call.price:.2f}")
    print(f"  Delta: {atm_call.delta:.4f}")
    print(f"\nPut Option:")
    print(f"  Price: ${atm_put.price:.2f}")
    print(f"  Delta: {atm_put.delta:.4f}")

    # Example 4: With dividends
    print("\n4. Option with Dividend Yield:")
    print("-" * 60)
    div_result = OptionPricer.price_call(
        spot_price=100.0,
        strike_price=100.0,
        time_to_expiry=1.0,
        risk_free_rate=0.05,
        volatility=0.2,
        dividend_yield=0.02,  # 2% dividend yield
    )
    print(f"Spot Price: $100.00")
    print(f"Strike Price: $100.00")
    print(f"Time to Expiry: 1 year")
    print(f"Risk-Free Rate: 5%")
    print(f"Volatility: 20%")
    print(f"Dividend Yield: 2%")
    print(f"\nResults:")
    print(f"  Option Price: ${div_result.price:.2f}")
    print(f"  Delta: {div_result.delta:.4f}")

    print("\n" + "=" * 60)


if __name__ == "__main__":
    main()
