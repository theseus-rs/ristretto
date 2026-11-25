public final class CanonicalInitializationOracle {
    static void row(String method, int argument, int actual, int expected) {
        if (actual != expected) throw new AssertionError(method + ": " + actual);
        System.out.println(method + " " + argument + " " + actual);
    }
    static void fault(String method) {
        try {
            if (method.equals("bad")) CanonicalClassInitialization.bad(0);
            else CanonicalClassInitialization.nestedBad(0);
            throw new AssertionError("missing initialization fault");
        } catch (ExceptionInInitializerError | NoClassDefFoundError expected) {
            System.out.println(method + " 0 " + expected.getClass().getSimpleName());
        }
    }
    public static void main(String[] args) {
        row("slow", 4, CanonicalClassInitialization.slow(4), 2096149);
        row("slow", -13, CanonicalClassInitialization.slow(-13), 2096132);
        row("storeSlow", 31, CanonicalClassInitialization.storeSlow(31), 31);
        row("returned", 0, CanonicalClassInitialization.returned(0), 31);
        row("returned", 0, CanonicalClassInitialization.returned(0), 0);
        row("nested", 0, CanonicalClassInitialization.nested(0), 2096146);
        row("cycleA", 0, CanonicalClassInitialization.cycleA(0), 3);
        row("cycleB", 0, CanonicalClassInitialization.cycleB(0), 2);
        fault("bad"); fault("bad"); fault("nestedBad"); fault("nestedBad");
        row("entry", 4, CanonicalEntryInitialization.run(4), 23);
    }
}
