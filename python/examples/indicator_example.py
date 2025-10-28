"""
Example usage of technical analysis indicators
"""

from finance_service import TechnicalIndicators


def main():
    """Demonstrate technical analysis indicators"""
    print("=" * 60)
    print("Technical Analysis Indicators Examples")
    print("=" * 60)

    # Example 1: Batch EMA calculation
    print("\n1. Batch EMA Calculation:")
    print("-" * 60)
    prices = [100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0, 110.0, 112.0]
    period = 5

    indicators = TechnicalIndicators()
    ema_values = indicators.ema(prices, period=period)

    print(f"Period: {period}")
    print(f"Number of prices: {len(prices)}")
    print(f"\nPrice | EMA")
    print("-" * 20)
    for i, (price, ema) in enumerate(zip(prices, ema_values), 1):
        if ema is None:
            print(f"{price:6.2f} | N/A (warming up)")
        else:
            print(f"{price:6.2f} | {ema:6.2f}")

    # Example 2: Streaming EMA calculation
    print("\n2. Streaming EMA Calculation (Real-time):")
    print("-" * 60)
    stream_prices = [100.0, 102.0, 101.0, 103.0, 105.0, 107.0, 106.0, 108.0]
    stream_period = 3

    calculator = indicators.ema_streaming(period=stream_period)
    print(f"Period: {stream_period}")
    print(f"Alpha (smoothing factor): {calculator.alpha:.4f}")
    print(f"\nSimulating real-time price updates:")
    print("Price | Updated EMA")
    print("-" * 25)

    for price in stream_prices:
        ema = calculator.update(price)
        print(f"{price:6.2f} | {ema:6.2f}")

    print(f"\nFinal EMA: {calculator.current_value:.2f}")

    # Example 3: Different EMA periods comparison
    print("\n3. Comparing Different EMA Periods:")
    print("-" * 60)
    long_prices = [
        100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0,
        107.0, 109.0, 110.0, 112.0, 111.0, 113.0, 115.0
    ]

    short_ema = indicators.ema(long_prices, period=3)
    medium_ema = indicators.ema(long_prices, period=5)
    long_ema = indicators.ema(long_prices, period=10)

    print(f"Comparing EMA-3, EMA-5, and EMA-10:")
    print(f"\nPrice  | EMA-3  | EMA-5  | EMA-10")
    print("-" * 45)

    for i, price in enumerate(long_prices):
        ema3 = short_ema[i]
        ema5 = medium_ema[i]
        ema10 = long_ema[i]

        ema3_str = f"{ema3:6.2f}" if ema3 is not None else "   N/A"
        ema5_str = f"{ema5:6.2f}" if ema5 is not None else "   N/A"
        ema10_str = f"{ema10:6.2f}" if ema10 is not None else "   N/A"

        print(f"{price:6.2f} | {ema3_str} | {ema5_str} | {ema10_str}")

    # Example 4: Trend detection using EMA
    print("\n4. Simple Trend Detection:")
    print("-" * 60)
    trend_prices = [100.0, 102.0, 105.0, 108.0, 110.0, 109.0, 107.0, 105.0, 103.0, 101.0]
    fast_period = 3
    slow_period = 5

    fast_ema = indicators.ema(trend_prices, period=fast_period)
    slow_ema = indicators.ema(trend_prices, period=slow_period)

    print(f"Fast EMA: {fast_period}-period")
    print(f"Slow EMA: {slow_period}-period")
    print(f"\nPrice  | Fast EMA | Slow EMA | Signal")
    print("-" * 50)

    for i, price in enumerate(trend_prices):
        fast = fast_ema[i]
        slow = slow_ema[i]

        if fast is not None and slow is not None:
            if fast > slow:
                signal = "BULLISH ↑"
            elif fast < slow:
                signal = "BEARISH ↓"
            else:
                signal = "NEUTRAL →"
            print(f"{price:6.2f} | {fast:8.2f} | {slow:8.2f} | {signal}")
        else:
            print(f"{price:6.2f} |     N/A  |     N/A  | Warming up...")

    print("\n" + "=" * 60)


if __name__ == "__main__":
    main()
