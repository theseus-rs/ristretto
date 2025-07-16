import java.lang.annotation.*;

// Test enum for array tests
enum TestEnum {
    FIRST, SECOND, THIRD
}

// Annotation for array testing
@Retention(RetentionPolicy.RUNTIME)
@interface ArrayTestAnnotation {
    String[] stringArray() default {};
    int[] intArray() default {};
    Class<?>[] classArray() default {};
    TestEnum[] enumArray() default {};
    SimpleNestedAnnotation[] annotationArray() default {};
}

// Simple nested annotation for array tests
@Retention(RetentionPolicy.RUNTIME)
@interface SimpleNestedAnnotation {
    String name();
    int value() default 0;
}

// Annotation with mixed array and single values
@Retention(RetentionPolicy.RUNTIME)
@interface MixedArrayAnnotation {
    String singleString() default "single";
    String[] stringArray() default {"default1", "default2"};
    int singleInt() default 42;
    int[] intArray() default {1, 2, 3};
}

// Test single element arrays vs direct values
@Retention(RetentionPolicy.RUNTIME)
@interface SingleElementArrayAnnotation {
    String[] singleElementArray() default {"single"};
    int[] singleIntArray() default {99};
    Class<?>[] singleClassArray() default {String.class};
}

@ArrayTestAnnotation(
        stringArray = {"first", "second", "third", ""},
        intArray = {1, -1, 0, Integer.MAX_VALUE, Integer.MIN_VALUE},
        classArray = {String.class, Integer.class, Object.class, void.class, int.class},
        enumArray = {TestEnum.FIRST, TestEnum.THIRD, TestEnum.SECOND},
        annotationArray = {
                @SimpleNestedAnnotation(name = "first", value = 1),
                @SimpleNestedAnnotation(name = "second", value = 2),
                @SimpleNestedAnnotation(name = "third")
        }
)
@MixedArrayAnnotation(
        singleString = "modified",
        stringArray = {"a", "b", "c", "d"},
        singleInt = 100,
        intArray = {10, 20, 30}
)
@SingleElementArrayAnnotation(
        singleElementArray = {"only one"},
        singleIntArray = {777},
        singleClassArray = {Double.class}
)
public class Test {

    // Test empty arrays
    @ArrayTestAnnotation
    private String emptyArraysField;

    // Test single-element arrays using array syntax
    @ArrayTestAnnotation(
            stringArray = {"single"},
            intArray = {42},
            classArray = {Test.class},
            enumArray = {TestEnum.SECOND},
            annotationArray = {@SimpleNestedAnnotation(name = "singleton")}
    )
    private String singleElementField;

    // Test arrays with special values
    @ArrayTestAnnotation(
            stringArray = {"", "  ", "\n", "\t", "normal"},
            intArray = {0, -0, 1, -1},
            classArray = {},
            enumArray = {},
            annotationArray = {}
    )
    public void specialValuesMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Arrays Test ===");

        Class<?> clazz = Test.class;

        // Test class-level annotations
        testClassArrayAnnotations(clazz);

        // Test empty arrays
        testEmptyArrays(clazz);

        // Test single element arrays
        testSingleElementArrays(clazz);

        // Test special values in arrays
        testSpecialValues(clazz);

        // Test array modification and immutability
        testArrayImmutability(clazz);

        // Test array equality and comparison
        testArrayEquality(clazz);
    }

    private static void testClassArrayAnnotations(Class<?> clazz) {
        System.out.println("=== Class Array Annotations ===");

        ArrayTestAnnotation arrayAnn = clazz.getAnnotation(ArrayTestAnnotation.class);
        if (arrayAnn != null) {
            // Test string array
            String[] stringArray = arrayAnn.stringArray();
            System.out.println("stringArray length: " + stringArray.length);
            System.out.print("stringArray values: [");
            for (int i = 0; i < stringArray.length; i++) {
                System.out.print("\"" + stringArray[i] + "\"");
                if (i < stringArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test int array
            int[] intArray = arrayAnn.intArray();
            System.out.println("intArray length: " + intArray.length);
            System.out.print("intArray values: [");
            for (int i = 0; i < intArray.length; i++) {
                System.out.print(intArray[i]);
                if (i < intArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test class array
            Class<?>[] classArray = arrayAnn.classArray();
            System.out.println("classArray length: " + classArray.length);
            System.out.print("classArray values: [");
            for (int i = 0; i < classArray.length; i++) {
                System.out.print(classArray[i].getSimpleName());
                if (i < classArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test enum array
            TestEnum[] enumArray = arrayAnn.enumArray();
            System.out.println("enumArray length: " + enumArray.length);
            System.out.print("enumArray values: [");
            for (int i = 0; i < enumArray.length; i++) {
                System.out.print(enumArray[i]);
                if (i < enumArray.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test annotation array
            SimpleNestedAnnotation[] annotationArray = arrayAnn.annotationArray();
            System.out.println("annotationArray length: " + annotationArray.length);
            for (int i = 0; i < annotationArray.length; i++) {
                SimpleNestedAnnotation nested = annotationArray[i];
                System.out.println("  [" + i + "] name: " + nested.name() + ", value: " + nested.value());
            }
        }

        // Test mixed array annotation
        MixedArrayAnnotation mixedAnn = clazz.getAnnotation(MixedArrayAnnotation.class);
        if (mixedAnn != null) {
            System.out.println("\nMixed Array Annotation:");
            System.out.println("singleString: " + mixedAnn.singleString());
            System.out.println("stringArray: " + java.util.Arrays.toString(mixedAnn.stringArray()));
            System.out.println("singleInt: " + mixedAnn.singleInt());
            System.out.println("intArray: " + java.util.Arrays.toString(mixedAnn.intArray()));
        }
    }

    private static void testEmptyArrays(Class<?> clazz) {
        System.out.println("\n=== Empty Arrays Test ===");

        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("emptyArraysField");
            ArrayTestAnnotation arrayAnn = field.getAnnotation(ArrayTestAnnotation.class);

            if (arrayAnn != null) {
                System.out.println("Empty stringArray length: " + arrayAnn.stringArray().length);
                System.out.println("Empty intArray length: " + arrayAnn.intArray().length);
                System.out.println("Empty classArray length: " + arrayAnn.classArray().length);
                System.out.println("Empty enumArray length: " + arrayAnn.enumArray().length);
                System.out.println("Empty annotationArray length: " + arrayAnn.annotationArray().length);

                // Test array identity for empty arrays
                String[] empty1 = arrayAnn.stringArray();
                String[] empty2 = arrayAnn.stringArray();
                System.out.println("Empty arrays same instance: " + (empty1 == empty2));
                System.out.println("Empty arrays equal: " + java.util.Arrays.equals(empty1, empty2));
            }
        } catch (Exception e) {
            System.out.println("Error testing empty arrays: " + e.getMessage());
        }
    }

    private static void testSingleElementArrays(Class<?> clazz) {
        System.out.println("\n=== Single Element Arrays Test ===");

        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("singleElementField");
            ArrayTestAnnotation arrayAnn = field.getAnnotation(ArrayTestAnnotation.class);

            if (arrayAnn != null) {
                String[] singleString = arrayAnn.stringArray();
                int[] singleInt = arrayAnn.intArray();
                Class<?>[] singleClass = arrayAnn.classArray();
                TestEnum[] singleEnum = arrayAnn.enumArray();
                SimpleNestedAnnotation[] singleAnnotation = arrayAnn.annotationArray();

                System.out.println("Single string array: " + java.util.Arrays.toString(singleString));
                System.out.println("Single int array: " + java.util.Arrays.toString(singleInt));
                System.out.println("Single class array: " + java.util.Arrays.toString(singleClass));
                System.out.println("Single enum array: " + java.util.Arrays.toString(singleEnum));
                System.out.println("Single annotation array length: " + singleAnnotation.length);
                if (singleAnnotation.length > 0) {
                    System.out.println("  annotation name: " + singleAnnotation[0].name());
                }
            }

            // Test SingleElementArrayAnnotation
            SingleElementArrayAnnotation singleElemAnn = clazz.getAnnotation(SingleElementArrayAnnotation.class);
            if (singleElemAnn != null) {
                System.out.println("SingleElementArray string: " +
                        java.util.Arrays.toString(singleElemAnn.singleElementArray()));
                System.out.println("SingleElementArray int: " +
                        java.util.Arrays.toString(singleElemAnn.singleIntArray()));
                System.out.println("SingleElementArray class: " +
                        java.util.Arrays.toString(singleElemAnn.singleClassArray()));
            }
        } catch (Exception e) {
            System.out.println("Error testing single element arrays: " + e.getMessage());
        }
    }

    private static void testSpecialValues(Class<?> clazz) {
        System.out.println("\n=== Special Values Test ===");

        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("specialValuesMethod");
            ArrayTestAnnotation arrayAnn = method.getAnnotation(ArrayTestAnnotation.class);

            if (arrayAnn != null) {
                String[] specialStrings = arrayAnn.stringArray();
                System.out.println("Special string array length: " + specialStrings.length);
                for (int i = 0; i < specialStrings.length; i++) {
                    String str = specialStrings[i];
                    System.out.println("  [" + i + "] = " +
                            (str == null ? "null" : "\"" + str.replace("\n", "\\n").replace("\t", "\\t") + "\"") +
                            " (length: " + (str == null ? "null" : str.length()) + ")");
                }

                int[] specialInts = arrayAnn.intArray();
                System.out.println("Special int array: " + java.util.Arrays.toString(specialInts));

                // Test zero and negative zero
                for (int i = 0; i < specialInts.length; i++) {
                    if (specialInts[i] == 0) {
                        System.out.println("  Found zero at index " + i);
                    }
                }
            }
        } catch (Exception e) {
            System.out.println("Error testing special values: " + e.getMessage());
        }
    }

    private static void testArrayImmutability(Class<?> clazz) {
        System.out.println("\n=== Array Immutability Test ===");

        ArrayTestAnnotation arrayAnn = clazz.getAnnotation(ArrayTestAnnotation.class);
        if (arrayAnn != null) {
            // Get arrays multiple times to test if they're the same instance
            String[] array1 = arrayAnn.stringArray();
            String[] array2 = arrayAnn.stringArray();

            System.out.println("Same array instance: " + (array1 == array2));
            System.out.println("Arrays equal: " + java.util.Arrays.equals(array1, array2));

            // Test modification (should not affect original)
            String originalFirst = array1[0];
            try {
                array1[0] = "modified";
                String[] array3 = arrayAnn.stringArray();
                System.out.println("Array modification affected annotation: " +
                        !array3[0].equals(originalFirst));
                System.out.println("First element after modification attempt: " + array3[0]);
            } catch (Exception e) {
                System.out.println("Exception during array modification: " + e.getClass().getSimpleName());
            }
        }
    }

    private static void testArrayEquality(Class<?> clazz) {
        System.out.println("\n=== Array Equality Test ===");

        ArrayTestAnnotation ann1 = clazz.getAnnotation(ArrayTestAnnotation.class);
        ArrayTestAnnotation ann2 = clazz.getAnnotation(ArrayTestAnnotation.class);

        if (ann1 != null && ann2 != null) {
            System.out.println("Annotation instances equal: " + ann1.equals(ann2));
            System.out.println("Annotation hash codes equal: " + (ann1.hashCode() == ann2.hashCode()));

            // Test array field equality
            String[] strings1 = ann1.stringArray();
            String[] strings2 = ann2.stringArray();

            System.out.println("String arrays equal: " + java.util.Arrays.equals(strings1, strings2));
            System.out.println("String arrays deep equal: " + java.util.Arrays.deepEquals(strings1, strings2));

            // Test nested annotation array equality
            SimpleNestedAnnotation[] nested1 = ann1.annotationArray();
            SimpleNestedAnnotation[] nested2 = ann2.annotationArray();

            System.out.println("Nested annotation arrays equal: " + java.util.Arrays.equals(nested1, nested2));

            if (nested1.length > 0 && nested2.length > 0) {
                System.out.println("First nested annotations equal: " + nested1[0].equals(nested2[0]));
            }
        }
    }
}
