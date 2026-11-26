import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Execute the exact class files exported by the native type-check continuation tests. */
public final class TypeCheckContinuationOracle {
    private static int checks;

    private static void require(boolean condition, String description) {
        if (!condition) throw new AssertionError(description);
        checks++;
    }

    public static void main(String[] args) throws Exception {
        Class<?> fixture = Class.forName("TypeCheckSemantics");
        Object[] values = {null, new int[] {77}, new long[] {78}, new Object(),
                new Object[1], new int[1][]};
        String[] labels = {"Ints", "Objects", "Matrix", "Object", "Cloneable",
                "Serializable", "Uninitialized"};
        boolean[][] expected = {
                {false, true, false, false, false, false},
                {false, false, false, false, true, true},
                {false, false, false, false, false, true},
                {false, true, true, true, true, true},
                {false, true, true, false, true, true},
                {false, true, true, false, true, true},
                {false, false, false, false, false, false},
        };
        for (int target = 0; target < labels.length; target++) {
            Method cast = fixture.getMethod("cast" + labels[target], Object.class);
            Method test = fixture.getMethod("test" + labels[target], Object.class);
            for (int value = 0; value < values.length; value++) {
                Object input = values[value];
                require(test.invoke(null, input).equals(expected[target][value] ? 1 : 0),
                        "instanceof " + labels[target] + " value " + value);
                try {
                    Object result = cast.invoke(null, input);
                    require((input == null || expected[target][value]) && result == input,
                            "checkcast " + labels[target] + " value " + value);
                } catch (InvocationTargetException error) {
                    require(input != null && !expected[target][value]
                                    && error.getCause() instanceof ClassCastException,
                            "cast error " + labels[target] + " value " + value);
                }
            }
        }
        require(fixture.getMethod("castMissing", Object.class).invoke(null, new Object[] {null}) == null,
                "null cast must not resolve missing target");
        require(fixture.getMethod("testMissing", Object.class).invoke(null, new Object[] {null}).equals(0),
                "null instanceof must not resolve missing target");
        Method wide = fixture.getMethod("wide", Object.class, long.class);
        for (long value : new long[] {Long.MIN_VALUE, -1, 0, 42, Long.MAX_VALUE}) {
            require(wide.invoke(null, values[1], value).equals(value * 2), "wide operand " + value);
        }
        Method caught = fixture.getMethod("caught", Object.class);
        require(caught.invoke(null, values[1]).equals(1), "successful protected cast");
        require(caught.invoke(null, values[2]).equals(0), "caught cast exception");
        System.out.println("type-check continuation oracle passed " + checks + " checks");
    }
}
