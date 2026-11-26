import java.lang.reflect.InvocationTargetException;

/** Verify exactly the class files exported by the constant continuation fixture. */
public final class ConstantOracle {
    private static int checks;

    private static void require(boolean condition, String description) {
        if (!condition) throw new AssertionError(description);
        checks++;
    }

    public static void main(String[] args) throws Exception {
        Class<?> fixture = Class.forName("ConstantSemantics");
        String text = (String) fixture.getMethod("text").invoke(null);
        require(text.equals("a\0b\ud83d\ude00"), "MUTF-8 text");
        require(text == text.intern(), "interned text identity");
        require(text == fixture.getMethod("text").invoke(null), "repeated text identity");
        String raw = (String) fixture.getMethod("surrogates").invoke(null);
        require(raw.length() == 3 && raw.charAt(0) == '\ud800'
                && raw.charAt(1) == 'a' && raw.charAt(2) == '\udfff', "lone surrogates");
        require(fixture.getMethod("wideText").invoke(null).equals("wide constant"), "wide string index");
        require(fixture.getMethod("wideClass").invoke(null) == Cloneable.class, "wide class index");
        require(fixture.getMethod("array").invoke(null) == int[][].class, "array class identity");
        Class<?> target = Class.forName("UninitializedConstantTarget", false, fixture.getClassLoader());
        // Its initializer loops forever: both calls must return without initialization.
        require(fixture.getMethod("target").invoke(null) == target, "class literal without initialization");
        require(fixture.getMethod("target").invoke(null) == target, "repeated mirror identity");
        for (long value : new long[] {Long.MIN_VALUE, -1, 0, 42, Long.MAX_VALUE}) {
            require(fixture.getMethod("wideResidual", long.class).invoke(null, value).equals(value * 2),
                    "category-two residual " + value);
        }
        try {
            fixture.getMethod("missing").invoke(null);
            throw new AssertionError("missing class literal returned");
        } catch (InvocationTargetException error) {
            require(error.getCause() instanceof NoClassDefFoundError, "resolution error kind");
        }
        require(fixture.getMethod("caught").invoke(null).equals(0), "caught resolution failure");
        for (boolean wide : new boolean[] {false, true}) {
            for (boolean string : new boolean[] {false, true}) {
                for (boolean cancel : new boolean[] {false, true}) {
                    Class<?> residual = Class.forName("ConstantWait" + wide + string + cancel);
                    int[] input = {77};
                    require(residual.getMethod("run", Object.class).invoke(null, input) == input,
                            "reference residual survives " + residual.getName());
                }
            }
        }
        System.out.println("constant oracle passed " + checks + " checks");
    }
}
