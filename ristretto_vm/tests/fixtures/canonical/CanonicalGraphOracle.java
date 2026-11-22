/** Run the shared classfiles on a stock JVM before comparing execution tiers. */
public final class CanonicalGraphOracle {
    public static void main(String[] args) {
        for (int rounds : new int[] {0, 1, 8}) {
            int result = CanonicalGraphWorkload.run(rounds);
            // Round i reads the two node values i and i+1 after unlinking.
            if (result != rounds * rounds) {
                throw new AssertionError("run(" + rounds + ") returned " + result);
            }
        }
        System.out.println("CanonicalGraphWorkload.run(8) = 64");
    }
}
