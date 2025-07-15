import java.util.Random;

public class Test {
    private static final Random random = new Random();

    // Monte Carlo estimation of π
    public static double estimatePi(int numSamples) {
        int pointsInCircle = 0;

        for (int i = 0; i < numSamples; i++) {
            double x = random.nextDouble() * 2 - 1; // Random between -1 and 1
            double y = random.nextDouble() * 2 - 1; // Random between -1 and 1

            if (x * x + y * y <= 1) {
                pointsInCircle++;
            }
        }

        return 4.0 * pointsInCircle / numSamples;
    }

    // Monte Carlo integration
    public static double integrate(Function function, double a, double b, int numSamples) {
        double sum = 0;

        for (int i = 0; i < numSamples; i++) {
            double x = a + random.nextDouble() * (b - a);
            sum += function.evaluate(x);
        }

        return (b - a) * sum / numSamples;
    }

    // Monte Carlo simulation for stock price (Geometric Brownian Motion)
    public static double[] simulateStockPrice(double initialPrice, double drift, double volatility,
                                            int timeSteps, double timeHorizon) {
        double[] prices = new double[timeSteps + 1];
        prices[0] = initialPrice;
        double dt = timeHorizon / timeSteps;

        for (int i = 1; i <= timeSteps; i++) {
            double randomShock = random.nextGaussian();
            double return_ = (drift - 0.5 * volatility * volatility) * dt + volatility * Math.sqrt(dt) * randomShock;
            prices[i] = prices[i - 1] * Math.exp(return_);
        }

        return prices;
    }

    // Monte Carlo option pricing (European Call)
    public static double priceEuropeanCall(double spotPrice, double strikePrice, double riskFreeRate,
                                         double volatility, double timeToMaturity, int numSimulations) {
        double payoffSum = 0;

        for (int i = 0; i < numSimulations; i++) {
            double randomShock = random.nextGaussian();
            double stockPriceAtMaturity = spotPrice * Math.exp(
                (riskFreeRate - 0.5 * volatility * volatility) * timeToMaturity +
                volatility * Math.sqrt(timeToMaturity) * randomShock
            );

            double payoff = Math.max(stockPriceAtMaturity - strikePrice, 0);
            payoffSum += payoff;
        }

        return Math.exp(-riskFreeRate * timeToMaturity) * payoffSum / numSimulations;
    }

    @FunctionalInterface
    interface Function {
        double evaluate(double x);
    }

    public static void main(String[] args) {
        System.out.println("Monte Carlo Methods");

        // Estimate π
        int piSamples = 1000000;
        double estimatedPi = estimatePi(piSamples);
        System.out.printf("Estimated π with %d samples: %.6f (actual: %.6f)%n",
                         piSamples, estimatedPi, Math.PI);
        System.out.printf("Error: %.6f%n", Math.abs(estimatedPi - Math.PI));

        // Monte Carlo integration of x^2 from 0 to 1 (should be 1/3)
        Function function = x -> x * x;
        double integral = integrate(function, 0, 1, 100000);
        System.out.printf("%nMonte Carlo integration of x² from 0 to 1: %.6f (actual: %.6f)%n",
                         integral, 1.0/3.0);

        // Stock price simulation
        double[] stockPrices = simulateStockPrice(100, 0.05, 0.2, 252, 1.0);
        System.out.printf("%nStock price simulation (1 year, daily steps):%n");
        System.out.printf("Initial price: $%.2f%n", stockPrices[0]);
        System.out.printf("Final price: $%.2f%n", stockPrices[stockPrices.length - 1]);

        // Option pricing
        double optionPrice = priceEuropeanCall(100, 105, 0.05, 0.2, 1.0, 100000);
        System.out.printf("%nEuropean call option price: $%.4f%n", optionPrice);
        System.out.println("(Spot: $100, Strike: $105, Rate: 5%, Vol: 20%, Time: 1 year)");
    }
}
