import java.lang.annotation.*;
import java.util.Arrays;
import java.util.Comparator;

// Test all valid annotation element types according to JLS
@Retention(RetentionPolicy.RUNTIME)
@interface AllElementTypesAnnotation {
    // Primitive types
    boolean booleanElement() default true;
    byte byteElement() default 1;
    short shortElement() default 2;
    int intElement() default 3;
    long longElement() default 4L;
    char charElement() default 'A';
    float floatElement() default 5.0f;
    double doubleElement() default 6.0;

    // String
    String stringElement() default "string";

    // Enum
    ElementType enumElement() default ElementType.TYPE;

    // Class
    Class<?> classElement() default Object.class;

    // Annotation type
    Retention annotationElement() default @Retention(RetentionPolicy.RUNTIME);

    // Arrays of the above
    boolean[] booleanArrayElement() default {true, false};
    byte[] byteArrayElement() default {1, 2, 3};
    short[] shortArrayElement() default {10, 20};
    int[] intArrayElement() default {100, 200, 300};
    long[] longArrayElement() default {1000L, 2000L};
    char[] charArrayElement() default {'X', 'Y', 'Z'};
    float[] floatArrayElement() default {1.1f, 2.2f};
    double[] doubleArrayElement() default {3.3, 4.4};
    String[] stringArrayElement() default {"one", "two", "three"};
    ElementType[] enumArrayElement() default {ElementType.FIELD, ElementType.METHOD};
    Class<?>[] classArrayElement() default {String.class, Integer.class};
    Retention[] annotationArrayElement() default {
        @Retention(RetentionPolicy.SOURCE),
        @Retention(RetentionPolicy.RUNTIME)
    };
}

// Test annotation for special target types
@Retention(RetentionPolicy.RUNTIME)
@Target({
    ElementType.ANNOTATION_TYPE,
    ElementType.CONSTRUCTOR,
    ElementType.FIELD,
    ElementType.LOCAL_VARIABLE,
    ElementType.METHOD,
    ElementType.PACKAGE,
    ElementType.PARAMETER,
    ElementType.TYPE,
    ElementType.TYPE_PARAMETER,
    ElementType.TYPE_USE
})
@interface AllTargetsAnnotation {
    String value() default "all_targets";
}

@AllElementTypesAnnotation(
    booleanElement = false,
    byteElement = 127,
    shortElement = 32767,
    intElement = 2147483647,
    longElement = 9223372036854775807L,
    charElement = '\u0041',
    floatElement = Float.MAX_VALUE,
    doubleElement = Double.MAX_VALUE,
    stringElement = "custom_string",
    enumElement = ElementType.FIELD,
    classElement = String.class,
    annotationElement = @Retention(RetentionPolicy.CLASS),
    booleanArrayElement = {false, true, false},
    byteArrayElement = {-128, 0, 127},
    shortArrayElement = {-32768, 0, 32767},
    intArrayElement = {Integer.MIN_VALUE, 0, Integer.MAX_VALUE},
    longArrayElement = {Long.MIN_VALUE, 0L, Long.MAX_VALUE},
    charArrayElement = {'\u0000', '\u007F', '\uFFFF'},
    floatArrayElement = {Float.MIN_VALUE, 0.0f, Float.MAX_VALUE, Float.NaN, Float.POSITIVE_INFINITY},
    doubleArrayElement = {Double.MIN_VALUE, 0.0, Double.MAX_VALUE, Double.NaN, Double.NEGATIVE_INFINITY},
    stringArrayElement = {"", "normal", "\u0048\u0065\u006C\u006C\u006F", "line1\nline2"},
    enumArrayElement = {ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER},
    classArrayElement = {void.class, boolean.class, byte.class, char.class, short.class, int.class, long.class, float.class, double.class},
    annotationArrayElement = {
        @Retention(RetentionPolicy.SOURCE),
        @Retention(RetentionPolicy.CLASS),
        @Retention(RetentionPolicy.RUNTIME)
    }
)
@AllTargetsAnnotation("class_level")
public class Test {

    @AllTargetsAnnotation("field_level")
    @AllElementTypesAnnotation(stringElement = "field", intElement = 999)
    private String testField;

    @AllTargetsAnnotation("constructor_level")
    public Test() {
    }

    @AllTargetsAnnotation("method_level")
    @AllElementTypesAnnotation(
        stringElement = "method",
        enumElement = ElementType.METHOD,
        classArrayElement = {Test.class}
    )
    public void testMethod(@AllTargetsAnnotation("parameter_level") String param) {
        @AllTargetsAnnotation("local_variable_level")
        String localVar = "test";
        System.out.println("Local variable: " + localVar);
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Element Types Test ===");

        Class<?> clazz = Test.class;

        // Test all element types on class
        testAllElementTypes(clazz);

        // Test target type coverage
        testTargetTypeCoverage(clazz);

        // Test element type validation
        testElementTypeValidation();

        // Test boundary values
        testBoundaryValues(clazz);

        // Test array element types
        testArrayElementTypes(clazz);
    }

    private static void testAllElementTypes(Class<?> clazz) {
        System.out.println("=== All Element Types Test ===");

        AllElementTypesAnnotation ann = clazz.getAnnotation(AllElementTypesAnnotation.class);
        if (ann != null) {
            // Test primitive elements
            System.out.println("booleanElement: " + ann.booleanElement());
            System.out.println("byteElement: " + ann.byteElement());
            System.out.println("shortElement: " + ann.shortElement());
            System.out.println("intElement: " + ann.intElement());
            System.out.println("longElement: " + ann.longElement());
            System.out.println("charElement: '" + ann.charElement() + "' (code: " + (int)ann.charElement() + ")");
            System.out.println("floatElement: " + ann.floatElement());
            System.out.println("doubleElement: " + ann.doubleElement());

            // Test string element
            System.out.println("stringElement: '" + ann.stringElement() + "'");

            // Test enum element
            System.out.println("enumElement: " + ann.enumElement());

            // Test class element
            System.out.println("classElement: " + ann.classElement().getName());

            // Test annotation element
            Retention retentionAnn = ann.annotationElement();
            System.out.println("annotationElement: " + retentionAnn.value());

            // Test array elements
            System.out.println("booleanArrayElement: " + java.util.Arrays.toString(ann.booleanArrayElement()));
            System.out.println("byteArrayElement: " + java.util.Arrays.toString(ann.byteArrayElement()));
            System.out.println("shortArrayElement: " + java.util.Arrays.toString(ann.shortArrayElement()));
            System.out.println("intArrayElement: " + java.util.Arrays.toString(ann.intArrayElement()));
            System.out.println("longArrayElement: " + java.util.Arrays.toString(ann.longArrayElement()));

            // Test char array with unicode
            char[] charArray = ann.charArrayElement();
            System.out.print("charArrayElement: [");
            for (int i = 0; i < charArray.length; i++) {
                System.out.print("'\\u" + String.format("%04X", (int)charArray[i]) + "'");
                if (i < charArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test float array with special values
            float[] floatArray = ann.floatArrayElement();
            System.out.print("floatArrayElement: [");
            for (int i = 0; i < floatArray.length; i++) {
                float f = floatArray[i];
                if (Float.isNaN(f)) {
                    System.out.print("NaN");
                } else if (Float.isInfinite(f)) {
                    System.out.print(f > 0 ? "POSITIVE_INFINITY" : "NEGATIVE_INFINITY");
                } else {
                    System.out.print(f);
                }
                if (i < floatArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test double array with special values
            double[] doubleArray = ann.doubleArrayElement();
            System.out.print("doubleArrayElement: [");
            for (int i = 0; i < doubleArray.length; i++) {
                double d = doubleArray[i];
                if (Double.isNaN(d)) {
                    System.out.print("NaN");
                } else if (Double.isInfinite(d)) {
                    System.out.print(d > 0 ? "POSITIVE_INFINITY" : "NEGATIVE_INFINITY");
                } else {
                    System.out.print(d);
                }
                if (i < doubleArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            System.out.println("stringArrayElement: " + java.util.Arrays.toString(ann.stringArrayElement()));
            System.out.println("enumArrayElement: " + java.util.Arrays.toString(ann.enumArrayElement()));

            Class<?>[] classArray = ann.classArrayElement();
            System.out.print("classArrayElement: [");
            for (int i = 0; i < classArray.length; i++) {
                System.out.print(classArray[i].getName());
                if (i < classArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            Retention[] annotationArray = ann.annotationArrayElement();
            System.out.print("annotationArrayElement: [");
            for (int i = 0; i < annotationArray.length; i++) {
                System.out.print(annotationArray[i].value());
                if (i < annotationArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");
        }
    }

    private static void testTargetTypeCoverage(Class<?> clazz) {
        System.out.println("\n=== Target Type Coverage Test ===");

        // Test class level
        AllTargetsAnnotation classTarget = clazz.getAnnotation(AllTargetsAnnotation.class);
        System.out.println("Class target: " + (classTarget != null ? classTarget.value() : "null"));

        // Test field level
        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("testField");
            AllTargetsAnnotation fieldTarget = field.getAnnotation(AllTargetsAnnotation.class);
            System.out.println("Field target: " + (fieldTarget != null ? fieldTarget.value() : "null"));
        } catch (Exception e) {
            System.out.println("Error accessing field: " + e.getMessage());
        }

        // Test constructor level
        try {
            java.lang.reflect.Constructor<?> constructor = clazz.getDeclaredConstructor();
            AllTargetsAnnotation constructorTarget = constructor.getAnnotation(AllTargetsAnnotation.class);
            System.out.println("Constructor target: " + (constructorTarget != null ? constructorTarget.value() : "null"));
        } catch (Exception e) {
            System.out.println("Error accessing constructor: " + e.getMessage());
        }

        // Test method level
        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("testMethod", String.class);
            AllTargetsAnnotation methodTarget = method.getAnnotation(AllTargetsAnnotation.class);
            System.out.println("Method target: " + (methodTarget != null ? methodTarget.value() : "null"));

            // Test parameter level
            Annotation[][] paramAnnotations = method.getParameterAnnotations();
            if (paramAnnotations.length > 0) {
                for (Annotation ann : paramAnnotations[0]) {
                    if (ann instanceof AllTargetsAnnotation) {
                        System.out.println("Parameter target: " + ((AllTargetsAnnotation) ann).value());
                    }
                }
            }
        } catch (Exception e) {
            System.out.println("Error accessing method: " + e.getMessage());
        }

        // Note: LOCAL_VARIABLE, PACKAGE, TYPE_PARAMETER, TYPE_USE targets
        // are not easily testable via runtime reflection
        System.out.println("Note: LOCAL_VARIABLE, PACKAGE, TYPE_PARAMETER, and TYPE_USE targets are not accessible via runtime reflection");
    }

    private static void testElementTypeValidation() {
        System.out.println("\n=== Element Type Validation Test ===");

        Class<AllElementTypesAnnotation> annClass = AllElementTypesAnnotation.class;
        java.lang.reflect.Method[] methods = annClass.getDeclaredMethods();
        // Sort methods by name for deterministic output
        Arrays.sort(methods, Comparator.comparing(java.lang.reflect.Method::getName));

        System.out.println("Annotation element methods: " + methods.length);

        for (java.lang.reflect.Method method : methods) {
            System.out.println("Element: " + method.getName());
            System.out.println("  Return type: " + method.getReturnType().getName());
            System.out.println("  Is array: " + method.getReturnType().isArray());

            if (method.getReturnType().isArray()) {
                Class<?> componentType = method.getReturnType().getComponentType();
                System.out.println("  Component type: " + componentType.getName());
                System.out.println("  Component is primitive: " + componentType.isPrimitive());
                System.out.println("  Component is enum: " + componentType.isEnum());
                System.out.println("  Component is annotation: " + componentType.isAnnotation());
            }

            Object defaultValue = method.getDefaultValue();
            System.out.println("  Has default: " + (defaultValue != null));
            if (defaultValue != null) {
                if (defaultValue.getClass().isArray()) {
                    if (defaultValue instanceof Object[]) {
                        System.out.println("  Default (array): " + java.util.Arrays.toString((Object[]) defaultValue));
                    } else {
                        // Handle primitive arrays
                        System.out.println("  Default (primitive array): " + java.lang.reflect.Array.getLength(defaultValue) + " elements");
                    }
                } else {
                    System.out.println("  Default: " + defaultValue);
                }
            }
        }
    }

    private static void testBoundaryValues(Class<?> clazz) {
        System.out.println("\n=== Boundary Values Test ===");

        AllElementTypesAnnotation ann = clazz.getAnnotation(AllElementTypesAnnotation.class);
        if (ann != null) {
            // Test boundary values for numeric types
            System.out.println("Byte value (127): " + ann.byteElement());
            System.out.println("Short value (32767): " + ann.shortElement());
            System.out.println("Int value (MAX_VALUE): " + ann.intElement());
            System.out.println("Long value (MAX_VALUE): " + ann.longElement());
            System.out.println("Float value (MAX_VALUE): " + ann.floatElement());
            System.out.println("Double value (MAX_VALUE): " + ann.doubleElement());

            // Test character boundary
            char charValue = ann.charElement();
            System.out.println("Char value: '" + charValue + "' (Unicode: U+" +
                             Integer.toHexString(charValue).toUpperCase() + ")");

            // Test array boundary values
            int[] intArray = ann.intArrayElement();
            System.out.println("Int array boundaries: MIN=" + intArray[0] + ", ZERO=" + intArray[1] + ", MAX=" + intArray[2]);

            long[] longArray = ann.longArrayElement();
            System.out.println("Long array boundaries: MIN=" + longArray[0] + ", ZERO=" + longArray[1] + ", MAX=" + longArray[2]);

            // Test special float values
            float[] floatArray = ann.floatArrayElement();
            System.out.println("Float special values:");
            for (int i = 0; i < floatArray.length; i++) {
                float f = floatArray[i];
                System.out.println("  [" + i + "] = " + f +
                                 " (isNaN: " + Float.isNaN(f) +
                                 ", isInfinite: " + Float.isInfinite(f) +
                                 ", isFinite: " + Float.isFinite(f) + ")");
            }
        }
    }

    private static void testArrayElementTypes(Class<?> clazz) {
        System.out.println("\n=== Array Element Types Test ===");

        AllElementTypesAnnotation ann = clazz.getAnnotation(AllElementTypesAnnotation.class);
        if (ann != null) {
            // Test that arrays are properly typed
            boolean[] booleanArray = ann.booleanArrayElement();
            System.out.println("Boolean array type: " + booleanArray.getClass().getName());
            System.out.println("Boolean array component type: " + booleanArray.getClass().getComponentType().getName());

            Class<?>[] classArray = ann.classArrayElement();
            System.out.println("Class array type: " + classArray.getClass().getName());
            System.out.println("Class array component type: " + classArray.getClass().getComponentType().getName());

            Retention[] annotationArray = ann.annotationArrayElement();
            System.out.println("Annotation array type: " + annotationArray.getClass().getName());
            System.out.println("Annotation array component type: " + annotationArray.getClass().getComponentType().getName());

            // Test array immutability
            String[] stringArray = ann.stringArrayElement();
            String originalFirst = stringArray[0];

            try {
                stringArray[0] = "modified";
                String[] stringArray2 = ann.stringArrayElement();
                System.out.println("Array modification test:");
                System.out.println("  Original first element: " + originalFirst);
                System.out.println("  Modified first element: " + stringArray[0]);
                System.out.println("  Fresh array first element: " + stringArray2[0]);
                System.out.println("  Array is immutable: " + stringArray2[0].equals(originalFirst));
            } catch (Exception e) {
                System.out.println("Array modification threw exception: " + e.getClass().getSimpleName());
            }
        }
    }
}
