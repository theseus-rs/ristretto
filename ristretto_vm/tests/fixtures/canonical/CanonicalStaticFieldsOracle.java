public final class CanonicalStaticFieldsOracle {
    static void row(String method, int argument, int actual, int expected) {
        if (actual != expected) throw new AssertionError(method + ": " + actual);
        System.out.println(method + " " + argument + " " + actual);
    }
    public static void main(String[] args) {
        row("read", 0, CanonicalStaticFields.read(0), 7);
        row("write", 11, CanonicalStaticFields.write(11), 29);
        row("read", 0, CanonicalStaticFields.read(0), 29);
        row("write", -3, CanonicalStaticFields.write(-3), 1);
        row("clear", 0, CanonicalStaticFields.clear(0), -3);
        row("read", 0, CanonicalStaticFields.read(0), 7);
        row("returned", 0, CanonicalStaticFields.returned(0), 0);
        row("write", 42, CanonicalStaticFields.write(42), 91);
        row("returned", 0, CanonicalStaticFields.returned(0), 42);
        row("read", 0, CanonicalStaticFields.read(0), 49);
        row("clear", 0, CanonicalStaticFields.clear(0), 0);
        row("read", 0, CanonicalStaticFields.read(0), 7);
        row("churn", 16, CanonicalStaticFields.churn(16), 120);
    }
}
