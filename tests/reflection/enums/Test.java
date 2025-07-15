/** Test enum reflection operations. */
public class Test {
    enum SimpleEnum {
        VALUE1, VALUE2, VALUE3
    }

    enum EnumWithFields {
        SMALL(1, "small"),
        MEDIUM(5, "medium"),
        LARGE(10, "large");

        private final int size;
        private final String description;

        EnumWithFields(int size, String description) {
            this.size = size;
            this.description = description;
        }

        public int getSize() { return size; }
        public String getDescription() { return description; }
    }

    enum EnumWithMethods {
        TYPE_A {
            @Override
            public void process() {
                System.out.println("Processing type A");
            }
        },
        TYPE_B {
            @Override
            public void process() {
                System.out.println("Processing type B");
            }
        };

        public abstract void process();

        public static void staticMethod() {
            System.out.println("Enum static method");
        }
    }

    public static void main(String[] args) throws Exception {
        // Test basic enum properties
        System.out.println("=== Basic Enum Properties ===");
        Class<?> simpleEnumClass = SimpleEnum.class;
        System.out.println("Is enum: " + simpleEnumClass.isEnum());
        System.out.println("Enum name: " + simpleEnumClass.getName());
        System.out.println("Enum superclass: " + simpleEnumClass.getSuperclass().getName());

        // Test enum constants
        System.out.println("\n=== Enum Constants ===");
        Object[] enumConstants = simpleEnumClass.getEnumConstants();
        System.out.println("Enum constants count: " + enumConstants.length);

        for (Object constant : enumConstants) {
            SimpleEnum enumValue = (SimpleEnum) constant;
            System.out.println("Enum constant: " + enumValue.name() + ", ordinal: " + enumValue.ordinal());
        }

        // Test enum with fields and constructor
        System.out.println("\n=== Enum with Fields ===");
        Class<?> enumWithFieldsClass = EnumWithFields.class;
        Object[] complexConstants = enumWithFieldsClass.getEnumConstants();

        for (Object constant : complexConstants) {
            EnumWithFields enumValue = (EnumWithFields) constant;
            System.out.println("Enum: " + enumValue.name() +
                ", size: " + enumValue.getSize() +
                ", description: " + enumValue.getDescription());
        }

        // Test enum fields
        java.lang.reflect.Field[] enumFields = enumWithFieldsClass.getDeclaredFields();
        System.out.println("Enum declared fields count: " + enumFields.length);

        for (java.lang.reflect.Field field : enumFields) {
            if (!field.isEnumConstant()) {
                System.out.println("Non-constant field: " + field.getName() +
                    " (" + field.getType().getName() + ")");
            }
        }

        // Test enum field access
        java.lang.reflect.Field sizeField = enumWithFieldsClass.getDeclaredField("size");
        sizeField.setAccessible(true);
        int sizeValue = sizeField.getInt(EnumWithFields.MEDIUM);
        System.out.println("MEDIUM size field value: " + sizeValue);

        // Test enum methods
        System.out.println("\n=== Enum Methods ===");
        java.lang.reflect.Method getSizeMethod = enumWithFieldsClass.getMethod("getSize");
        Object result = getSizeMethod.invoke(EnumWithFields.LARGE);
        System.out.println("LARGE getSize() result: " + result);

        // Test enum constructor
        System.out.println("\n=== Enum Constructor ===");
        java.lang.reflect.Constructor<?>[] constructors = enumWithFieldsClass.getDeclaredConstructors();
        System.out.println("Enum constructors count: " + constructors.length);

        for (java.lang.reflect.Constructor<?> constructor : constructors) {
            System.out.println("Constructor parameters: " + constructor.getParameterCount());
            Class<?>[] paramTypes = constructor.getParameterTypes();
            for (int i = 0; i < paramTypes.length; i++) {
                System.out.println("  Parameter " + i + ": " + paramTypes[i].getName());
            }
        }

        // Test enum with abstract methods
        System.out.println("\n=== Enum with Abstract Methods ===");
        Class<?> enumWithMethodsClass = EnumWithMethods.class;
        Object[] methodConstants = enumWithMethodsClass.getEnumConstants();

        for (Object constant : methodConstants) {
            EnumWithMethods enumValue = (EnumWithMethods) constant;
            System.out.println("Processing enum: " + enumValue.name());
            enumValue.process();

            // Check the actual class of the enum constant
            Class<?> constantClass = constant.getClass();
            System.out.println("Constant class: " + constantClass.getName());
            System.out.println("Is anonymous: " + constantClass.isAnonymousClass());
        }

        // Test enum static method
        java.lang.reflect.Method staticMethod = enumWithMethodsClass.getMethod("staticMethod");
        staticMethod.invoke(null);

        // Test enum valueOf and values methods
        System.out.println("\n=== Enum Special Methods ===");
        java.lang.reflect.Method valueOfMethod = simpleEnumClass.getMethod("valueOf", String.class);
        Object valueOfResult = valueOfMethod.invoke(null, "VALUE2");
        System.out.println("valueOf result: " + valueOfResult);

        java.lang.reflect.Method valuesMethod = simpleEnumClass.getMethod("values");
        Object[] valuesResult = (Object[]) valuesMethod.invoke(null);
        System.out.println("values() result length: " + valuesResult.length);

        // Test enum comparison methods
        System.out.println("\n=== Enum Comparison ===");
        SimpleEnum enum1 = SimpleEnum.VALUE1;
        SimpleEnum enum2 = SimpleEnum.VALUE2;

        java.lang.reflect.Method compareToMethod = simpleEnumClass.getMethod("compareTo", Enum.class);
        int compareResult = (Integer) compareToMethod.invoke(enum1, enum2);
        System.out.println("VALUE1.compareTo(VALUE2): " + compareResult);

        // Test enum field isEnumConstant
        System.out.println("\n=== Enum Constant Fields ===");
        java.lang.reflect.Field[] allFields = simpleEnumClass.getDeclaredFields();
        for (java.lang.reflect.Field field : allFields) {
            if (field.isEnumConstant()) {
                System.out.println("Enum constant field: " + field.getName());
            }
        }

        // Test enum annotation support
        System.out.println("\n=== Enum Annotations ===");
        java.lang.annotation.Annotation[] enumAnnotations = simpleEnumClass.getAnnotations();
        System.out.println("Enum class annotations: " + enumAnnotations.length);

        // Test enum reflection edge cases
        System.out.println("\n=== Enum Edge Cases ===");
        try {
            // Try to create enum instance (should fail)
            java.lang.reflect.Constructor<?> enumConstructor = enumWithFieldsClass.getDeclaredConstructor(String.class, int.class, int.class, String.class);
            enumConstructor.setAccessible(true);
            System.out.println("ERROR: Should not be able to create enum constructor");
        } catch (NoSuchMethodException e) {
            System.out.println("Correctly failed to access enum constructor directly");
        } catch (Exception e) {
            System.out.println("Failed to create enum instance: " + e.getClass().getSimpleName());
        }

        // Test enum class hierarchy
        System.out.println("Enum superclass is Enum: " + (simpleEnumClass.getSuperclass() == Enum.class));
        System.out.println("Enum implements Comparable: " + Comparable.class.isAssignableFrom(simpleEnumClass));
        System.out.println("Enum implements Serializable: " + java.io.Serializable.class.isAssignableFrom(simpleEnumClass));
    }
}
