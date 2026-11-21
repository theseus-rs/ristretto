public final class WideCallsOracle {
    public static void main(String[] args) {
        for (int value : new int[] {Integer.MIN_VALUE, -16777217, -65536, -1, 0, 1,
                                    127, 65535, 16777217, Integer.MAX_VALUE}) {
            System.out.println("combine\t" + value + "\t" + CanonicalWideCalls.combine(value));
        }
    }
}
