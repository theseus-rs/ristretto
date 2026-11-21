public final class CanonicalCompiledCallsOracle {
    public static void main(String[] args) {
        for (int value : new int[] {-1, 0, 1, 9, 64}) {
            System.out.println("run\t" + value + "\t" + CanonicalCompiledCalls.run(value));
            System.out.println("depth\t" + value + "\t" + CanonicalCompiledCalls.depth(value));
        }
    }
}
