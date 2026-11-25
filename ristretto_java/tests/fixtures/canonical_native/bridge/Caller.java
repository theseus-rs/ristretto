package bridge;

/** Ordinary launcher entry. Only enter(int) is explicitly bound. */
public final class Caller {
    static { System.setProperty("canonical.bridge.bootstrap", "ordinary"); }
    private static native int enter(int value);
    private native int instance(int value);
    private static native long wrong(long value);
    private static int ordinary(int value) { return value; }

    public static void main(String[] args) {
        if (!"ordinary".equals(System.getProperty("canonical.bridge.bootstrap"))) {
            throw new AssertionError("ordinary class initialization was skipped");
        }
        if (args.length != 2 || !"hello".equals(args[0])
                || !"--canonical-native=application-argument".equals(args[1])) {
            throw new AssertionError("ordinary main arguments changed");
        }
        System.out.println("ordinary-before:hello");
        int first = enter(7);
        int second = enter(11);
        if (first != 25 || second != 37) {
            throw new AssertionError("canonical bridge result changed");
        }
        boolean caught = false;
        try {
            enter(-1);
        } catch (NegativeArraySizeException expected) {
            caught = true;
            System.out.println("ordinary-caught:NegativeArraySizeException");
        }
        if (!caught) {
            throw new AssertionError("canonical runtime exception did not cross the boundary");
        }
        System.out.println("ordinary-after:" + first + "," + second);
    }
}
