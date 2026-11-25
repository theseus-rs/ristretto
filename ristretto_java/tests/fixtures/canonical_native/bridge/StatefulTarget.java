package bridge;

/** Denied by this boundary's stateless admission contract. */
public final class StatefulTarget {
    private static int total;
    public static int compute(int value) {
        total += value;
        return total;
    }
}
