import java.lang.annotation.*;
import java.lang.reflect.*;

// Annotation with complex default values
@Retention(RetentionPolicy.RUNTIME)
@interface ComplexAnnotation {
    String[] emptyArray() default {};
    String[] singleElementArray() default {"single"};
    Class<?> voidClass() default void.class;
    Class<?> primitiveClass() default int.class;
    String nullableString() default "";
    int maxInt() default Integer.MAX_VALUE;
    int minInt() default Integer.MIN_VALUE;
    long maxLong() default Long.MAX_VALUE;
    double maxDouble() default Double.MAX_VALUE;
    double minDouble() default Double.MIN_VALUE;
    double nanDouble() default Double.NaN;
    double posInfinity() default Double.POSITIVE_INFINITY;
    double negInfinity() default Double.NEGATIVE_INFINITY;
    float nanFloat() default Float.NaN;
}

// Annotation with special characters in strings
@Retention(RetentionPolicy.RUNTIME)
@interface SpecialChars {
    String unicode() default "\u0041\u0042\u0043"; // ABC
    String newlines() default "line1\nline2\r\nline3";
    String tabs() default "col1\tcol2\tcol3";
    String quotes() default "He said \"Hello\" to 'everyone'";
    String backslashes() default "path\\to\\file";
    String empty() default "";
}

// Deeply nested annotation
@Retention(RetentionPolicy.RUNTIME)
@interface Level1 {
    Level2 nested();
}

@Retention(RetentionPolicy.RUNTIME)
@interface Level2 {
    Level3 nested();
}

@Retention(RetentionPolicy.RUNTIME)
@interface Level3 {
    String value();
}

// Annotation without circular reference
@Retention(RetentionPolicy.RUNTIME)
@interface SelfReferencing {
    String name();
    // Removed children to avoid cyclic reference
}

// Annotation for testing null and edge cases
@Retention(RetentionPolicy.RUNTIME)
@interface EdgeCaseAnnotation {
    String normalString() default "normal";
    String[] stringArray() default {"a", "b", "c"};
    Class<?>[] classArray() default {String.class, Object.class};
}

@ComplexAnnotation(
        emptyArray = {},
        singleElementArray = {"modified"},
        voidClass = Void.class,
        primitiveClass = boolean.class,
        nullableString = "not empty",
        maxInt = -1,
        minInt = 1,
        maxLong = 0L,
        maxDouble = 1.23,
        minDouble = 4.56,
        nanDouble = 42.0,
        posInfinity = 7.89,
        negInfinity = -1.23,
        nanFloat = 3.14f
)
@SpecialChars(
        unicode = "\u4E2D\u6587\u6D4B\u8BD5", // Chinese characters
        newlines = "first\nsecond\rthird",
        tabs = "a\tb\tc",
        quotes = "\"quoted\" and 'apostrophe'",
        backslashes = "C:\\Windows\\System32",
        empty = ""
)
@Level1(nested = @Level2(nested = @Level3("deep value")))
@SelfReferencing(
        name = "root"
        // Removed children to avoid cyclic reference
)
@EdgeCaseAnnotation
public class Test {

    // Test annotation on synthetic/bridge methods (if any)
    // Test annotation inheritance with generics
    static class GenericBase<T> {
        @ComplexAnnotation
        public void genericMethod(T param) {}
    }

    static class ConcreteChild extends GenericBase<String> {
        @Override
        public void genericMethod(String param) {}
    }

    public static void main(String[] args) {
        System.out.println("=== Edge Cases and Complex Scenarios Test ===");

        Class<?> clazz = Test.class;

        // Test complex default values
        testComplexAnnotation(clazz);

        // Test special characters
        testSpecialCharacters(clazz);

        // Test nested annotations
        testNestedAnnotations(clazz);

        // Test self-referencing annotations
        testSelfReferencingAnnotations(clazz);

        // Test edge case annotation
        testEdgeCaseAnnotation(clazz);

        // Test generic method annotations
        testGenericMethodAnnotations();

        // Test annotation on array types
        testArrayTypeAnnotations();

        // Test annotation reflection edge cases
        testReflectionEdgeCases(clazz);
    }

    private static void testComplexAnnotation(Class<?> clazz) {
        System.out.println("=== Complex Annotation Test ===");
        ComplexAnnotation ann = clazz.getAnnotation(ComplexAnnotation.class);
        if (ann != null) {
            System.out.println("emptyArray length: " + ann.emptyArray().length);
            System.out.println("singleElementArray: " + java.util.Arrays.toString(ann.singleElementArray()));
            System.out.println("voidClass: " + ann.voidClass().getName());
            System.out.println("primitiveClass: " + ann.primitiveClass().getName());
            System.out.println("nullableString: '" + ann.nullableString() + "'");
            System.out.println("maxInt: " + ann.maxInt());
            System.out.println("minInt: " + ann.minInt());
            System.out.println("maxLong: " + ann.maxLong());
            System.out.println("maxDouble: " + ann.maxDouble());
            System.out.println("minDouble: " + ann.minDouble());
            System.out.println("nanDouble: " + ann.nanDouble());
            System.out.println("posInfinity: " + ann.posInfinity());
            System.out.println("negInfinity: " + ann.negInfinity());
            System.out.println("nanFloat: " + ann.nanFloat());

            // Test special float/double values
            System.out.println("nanDouble is NaN: " + (ann.nanDouble() != ann.nanDouble())); // NaN != NaN
            System.out.println("posInfinity is finite: " + Double.isFinite(ann.posInfinity()));
            System.out.println("negInfinity is infinite: " + Double.isInfinite(ann.negInfinity()));
        }
    }

    private static void testSpecialCharacters(Class<?> clazz) {
        System.out.println("\n=== Special Characters Test ===");
        SpecialChars ann = clazz.getAnnotation(SpecialChars.class);
        if (ann != null) {
            System.out.println("unicode: " + ann.unicode());
            System.out.println("unicode length: " + ann.unicode().length());
            System.out.println("newlines contains \\n: " + ann.newlines().contains("\n"));
            System.out.println("tabs contains \\t: " + ann.tabs().contains("\t"));
            System.out.println("quotes: " + ann.quotes());
            System.out.println("backslashes: " + ann.backslashes());
            System.out.println("empty length: " + ann.empty().length());
            System.out.println("empty isEmpty: " + ann.empty().isEmpty());
        }
    }

    private static void testNestedAnnotations(Class<?> clazz) {
        System.out.println("\n=== Nested Annotations Test ===");
        Level1 ann = clazz.getAnnotation(Level1.class);
        if (ann != null) {
            Level2 level2 = ann.nested();
            System.out.println("Level2 found: " + (level2 != null));
            if (level2 != null) {
                Level3 level3 = level2.nested();
                System.out.println("Level3 found: " + (level3 != null));
                if (level3 != null) {
                    System.out.println("Level3 value: " + level3.value());
                }
            }
        }
    }

    private static void testSelfReferencingAnnotations(Class<?> clazz) {
        System.out.println("\n=== Self-Referencing Annotations Test ===");
        SelfReferencing ann = clazz.getAnnotation(SelfReferencing.class);
        if (ann != null) {
            System.out.println("Root name: " + ann.name());
            // Removed children handling
        }
    }

    private static void testEdgeCaseAnnotation(Class<?> clazz) {
        System.out.println("\n=== Edge Case Annotation Test ===");
        EdgeCaseAnnotation ann = clazz.getAnnotation(EdgeCaseAnnotation.class);
        if (ann != null) {
            System.out.println("normalString: " + ann.normalString());
            System.out.println("stringArray: " + java.util.Arrays.toString(ann.stringArray()));

            Class<?>[] classArray = ann.classArray();
            System.out.print("classArray: [");
            for (int i = 0; i < classArray.length; i++) {
                System.out.print(classArray[i].getName());
                if (i < classArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");
        }
    }

    private static void testGenericMethodAnnotations() {
        System.out.println("\n=== Generic Method Annotations Test ===");
        try {
            Method baseMethod = GenericBase.class.getDeclaredMethod("genericMethod", Object.class);
            Method childMethod = ConcreteChild.class.getDeclaredMethod("genericMethod", String.class);

            System.out.println("Base method has ComplexAnnotation: " +
                    baseMethod.isAnnotationPresent(ComplexAnnotation.class));
            System.out.println("Child method has ComplexAnnotation: " +
                    childMethod.isAnnotationPresent(ComplexAnnotation.class));

            // Test bridge method (if created)
            Method[] childMethods = ConcreteChild.class.getDeclaredMethods();
            System.out.println("ConcreteChild method count: " + childMethods.length);
            for (Method m : childMethods) {
                System.out.println("  Method: " + m.getName() + ", bridge: " + m.isBridge() +
                        ", synthetic: " + m.isSynthetic());
            }
        } catch (Exception e) {
            System.out.println("Error testing generic methods: " + e.getMessage());
        }
    }

    private static void testArrayTypeAnnotations() {
        System.out.println("\n=== Array Type Annotations Test ===");
        // Note: Type annotations on arrays are a complex topic
        // This tests basic array handling in annotation values

        Class<?> stringArrayClass = String[].class;
        Class<?> intArrayClass = int[].class;

        System.out.println("String[] is array: " + stringArrayClass.isArray());
        System.out.println("int[] is array: " + intArrayClass.isArray());
        System.out.println("String[] component type: " + stringArrayClass.getComponentType().getName());
        System.out.println("int[] component type: " + intArrayClass.getComponentType().getName());
    }

    private static void testReflectionEdgeCases(Class<?> clazz) {
        System.out.println("\n=== Reflection Edge Cases Test ===");

        // Test annotation on class multiple times
        Annotation[] annotations = clazz.getAnnotations();
        System.out.println("Total annotations: " + annotations.length);

        // Test getDeclaredAnnotations vs getAnnotations
        Annotation[] declared = clazz.getDeclaredAnnotations();
        System.out.println("Declared annotations: " + declared.length);
        System.out.println("Same count: " + (annotations.length == declared.length));

        // Test null safety
        try {
            ComplexAnnotation nullTest = clazz.getAnnotation(null);
            System.out.println("Getting annotation with null class: " + nullTest);
        } catch (Exception e) {
            System.out.println("Exception with null annotation class: " + e.getClass().getSimpleName());
        }

        // Test annotation methods
        for (Annotation ann : annotations) {
            Class<? extends Annotation> annType = ann.annotationType();
            System.out.println("Annotation type: " + annType.getSimpleName());
            System.out.println("  Is annotation: " + annType.isAnnotation());
            System.out.println("  Is interface: " + annType.isInterface());
            System.out.println("  Method count: " + annType.getDeclaredMethods().length);
        }
    }
}
