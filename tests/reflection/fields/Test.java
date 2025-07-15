/** Test field reflection operations. */
public class Test {
    static class A {
        public static final String STATIC_FINAL_FIELD = "constant";
        public static String staticField = "static";
        public String publicField = "public";
        protected String protectedField = "protected";
        private String privateField = "private";
        String packageField = "package";

        public final String finalField = "final";
        public transient String transientField = "transient";
        public volatile String volatileField = "volatile";

        public int intField = 42;
        public double doubleField = 3.14;
        public boolean booleanField = true;
        public String[] arrayField = {"a", "b", "c"};

        // Field with generic type
        public java.util.List<String> genericField;
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Class.forName("Test$A");
        A instance = new A();

        // Test getting all fields
        java.lang.reflect.Field[] fields = clazz.getDeclaredFields();
        System.out.println("Declared fields count: " + fields.length);

        // Test getting public fields (includes inherited)
        java.lang.reflect.Field[] publicFields = clazz.getFields();
        System.out.println("Public fields count: " + publicFields.length);

        // Test getting specific field
        java.lang.reflect.Field publicField = clazz.getField("publicField");
        System.out.println("Found public field: " + publicField.getName());

        // Test field type
        System.out.println("Public field type: " + publicField.getType().getName());

        // Test field modifiers
        java.lang.reflect.Field staticFinalField = clazz.getField("STATIC_FINAL_FIELD");
        System.out.println("Static final field modifiers: " + java.lang.reflect.Modifier.toString(staticFinalField.getModifiers()));

        // Test field value access
        Object value = publicField.get(instance);
        System.out.println("Public field value: " + value);

        // Test static field access
        java.lang.reflect.Field staticField = clazz.getField("staticField");
        Object staticValue = staticField.get(null);
        System.out.println("Static field value: " + staticValue);

        // Test field value setting
        publicField.set(instance, "modified");
        System.out.println("Modified public field value: " + publicField.get(instance));

        // Test primitive field access
        java.lang.reflect.Field intField = clazz.getField("intField");
        System.out.println("Int field value: " + intField.getInt(instance));

        java.lang.reflect.Field doubleField = clazz.getField("doubleField");
        System.out.println("Double field value: " + doubleField.getDouble(instance));

        java.lang.reflect.Field booleanField = clazz.getField("booleanField");
        System.out.println("Boolean field value: " + booleanField.getBoolean(instance));

        // Test array field access
        java.lang.reflect.Field arrayField = clazz.getField("arrayField");
        String[] arrayValue = (String[]) arrayField.get(instance);
        System.out.println("Array field length: " + arrayValue.length);

        // Test private field access with setAccessible
        java.lang.reflect.Field privateField = clazz.getDeclaredField("privateField");
        privateField.setAccessible(true);
        System.out.println("Private field value: " + privateField.get(instance));

        // Test field accessibility
        System.out.println("Public field accessible: " + publicField.isAccessible());
        System.out.println("Private field accessible after setAccessible: " + privateField.isAccessible());
    }
}

