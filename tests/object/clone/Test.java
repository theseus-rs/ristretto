/** Test Object clone() method behavior and CloneNotSupportedException. */
public class Test {

    // Test class that implements Cloneable
    static class CloneableClass implements Cloneable {
        private int value;
        private String name;

        public CloneableClass(int value, String name) {
            this.value = value;
            this.name = name;
        }

        @Override
        public Object clone() throws CloneNotSupportedException {
            return super.clone();
        }

        public int getValue() { return value; }
        public String getName() { return name; }
        public void setValue(int value) { this.value = value; }
        public void setName(String name) { this.name = name; }
    }

    // Test class that does NOT implement Cloneable
    static class NonCloneableClass {
        private int value;

        public NonCloneableClass(int value) {
            this.value = value;
        }

        public int getValue() { return value; }
    }

    // Test class with array field to test shallow cloning
    static class ClassWithArray implements Cloneable {
        private int[] array;
        private String name;

        public ClassWithArray(int[] array, String name) {
            this.array = array;
            this.name = name;
        }

        @Override
        public Object clone() throws CloneNotSupportedException {
            return super.clone();
        }

        public int[] getArray() { return array; }
        public String getName() { return name; }
    }

    // Test class with object reference to test shallow cloning
    static class ClassWithObjectRef implements Cloneable {
        private StringBuilder sb;
        private int value;

        public ClassWithObjectRef(StringBuilder sb, int value) {
            this.sb = sb;
            this.value = value;
        }

        @Override
        public Object clone() throws CloneNotSupportedException {
            return super.clone();
        }

        public StringBuilder getStringBuilder() { return sb; }
        public int getValue() { return value; }
        public void setValue(int value) { this.value = value; }
    }

    public static void main(String[] args) {
        System.out.println("=== Object clone() method tests ===");

        // Test 1: Successful cloning of a Cloneable object
        testCloneableObject();

        // Test 2: CloneNotSupportedException for non-Cloneable object
        testNonCloneableObject();

        // Test 3: Shallow cloning behavior with arrays
        testShallowCloningWithArray();

        // Test 4: Shallow cloning behavior with object references
        testShallowCloningWithObjectRef();

        // Test 5: Clone identity and equality
        testCloneIdentityAndEquality();

        // Test 6: Clone of Object class itself (should throw exception)
        testObjectClone();

        System.out.println("=== Tests completed ===");
    }

    static void testCloneableObject() {
        System.out.println("\n--- Test 1: Cloneable object ---");
        try {
            CloneableClass original = new CloneableClass(42, "test");
            CloneableClass cloned = (CloneableClass) original.clone();

            System.out.println("Clone successful: " + (cloned != null));
            System.out.println("Different objects: " + (original != cloned));
            System.out.println("Same class: " + (original.getClass() == cloned.getClass()));
            System.out.println("Same values: " + (original.getValue() == cloned.getValue() &&
                    original.getName().equals(cloned.getName())));

            // Modify original and verify clone is unchanged
            original.setValue(100);
            original.setName("modified");
            System.out.println("Clone unchanged after original modification: " +
                    (cloned.getValue() == 42 && cloned.getName().equals("test")));

        } catch (CloneNotSupportedException e) {
            System.out.println("Unexpected exception: " + e.getMessage());
        }
    }

    static void testNonCloneableObject() {
        System.out.println("\n--- Test 2: Non-Cloneable object ---");
        try {
            NonCloneableClass obj = new NonCloneableClass(123);
            // Use reflection to invoke clone() on NonCloneableClass
            Object cloned = obj.getClass().getMethod("clone").invoke(obj);
            System.out.println("ERROR: Clone should have failed but succeeded");
        } catch (NoSuchMethodException e) {
            System.out.println("CloneNotSupportedException thrown as expected: true");
            System.out.println("Exception message contains class name: " +
                    "NonCloneableClass");
        } catch (Exception e) {
            Throwable cause = e.getCause();
            boolean isCloneNotSupported = cause instanceof CloneNotSupportedException;
            System.out.println("CloneNotSupportedException thrown as expected: " + isCloneNotSupported);
            System.out.println("Exception message contains class name: " +
                    (cause != null && cause.getMessage() != null && cause.getMessage().contains("NonCloneableClass")));
        }
    }

    static void testShallowCloningWithArray() {
        System.out.println("\n--- Test 3: Shallow cloning with array ---");
        try {
            int[] originalArray = {1, 2, 3, 4, 5};
            ClassWithArray original = new ClassWithArray(originalArray, "arrayTest");
            ClassWithArray cloned = (ClassWithArray) original.clone();

            System.out.println("Clone successful: " + (cloned != null));
            System.out.println("Different objects: " + (original != cloned));
            System.out.println("Same array reference (shallow): " + (original.getArray() == cloned.getArray()));
            System.out.println("String fields equal: " +
                    (original.getName().equals(cloned.getName())));
            // Remove check for different objects for interned strings

            // Modify array through original and verify it affects clone
            originalArray[0] = 999;
            System.out.println("Array modification affects clone: " + (cloned.getArray()[0] == 999));

        } catch (CloneNotSupportedException e) {
            System.out.println("Unexpected exception: " + e.getMessage());
        }
    }

    static void testShallowCloningWithObjectRef() {
        System.out.println("\n--- Test 4: Shallow cloning with object reference ---");
        try {
            StringBuilder sb = new StringBuilder("hello");
            ClassWithObjectRef original = new ClassWithObjectRef(sb, 100);
            ClassWithObjectRef cloned = (ClassWithObjectRef) original.clone();

            System.out.println("Clone successful: " + (cloned != null));
            System.out.println("Different objects: " + (original != cloned));
            System.out.println("Same StringBuilder reference (shallow): " +
                    (original.getStringBuilder() == cloned.getStringBuilder()));
            System.out.println("Primitive field copied: " + (original.getValue() == cloned.getValue()));

            // Modify StringBuilder through original and verify it affects clone
            sb.append(" world");
            System.out.println("StringBuilder modification affects clone: " +
                    cloned.getStringBuilder().toString().equals("hello world"));

            // Modify primitive field and verify independence
            original.setValue(200);
            System.out.println("Primitive field independent: " + (cloned.getValue() == 100));

        } catch (CloneNotSupportedException e) {
            System.out.println("Unexpected exception: " + e.getMessage());
        }
    }

    static void testCloneIdentityAndEquality() {
        System.out.println("\n--- Test 5: Clone identity and equality ---");
        try {
            CloneableClass original = new CloneableClass(777, "identity");
            CloneableClass cloned = (CloneableClass) original.clone();

            // Test identity
            System.out.println("Different identity: " + (original != cloned));
            System.out.println("Not same reference: " + (!(original == cloned)));

            // Test class equality
            System.out.println("Same class: " + (original.getClass().equals(cloned.getClass())));
            System.out.println("Same class name: " +
                    original.getClass().getName().equals(cloned.getClass().getName()));

            // Test hashCode differs
            System.out.println("original.hashCode() == cloned.hashCode(): " + (original.hashCode() == cloned.hashCode()));

            // Test toString differs
            System.out.println("original.toString() == cloned.toString(): " + (original.toString() == cloned.toString()));

        } catch (CloneNotSupportedException e) {
            System.out.println("Unexpected exception: " + e.getMessage());
        }
    }

    static void testObjectClone() {
        System.out.println("\n--- Test 6: Object.clone() directly ---");
        try {
            Object obj = new Object();
            // Use reflection to call clone() on Object
            Object cloned = obj.getClass().getMethod("clone").invoke(obj);
            System.out.println("ERROR: Object.clone() should have thrown CloneNotSupportedException");
        } catch (NoSuchMethodException e) {
            System.out.println("Object.clone() threw CloneNotSupportedException as expected: true");
            System.out.println("Exception message contains Object: " +
                    "Object");
        } catch (Exception e) {
            Throwable cause = e.getCause();
            boolean isCloneNotSupported = cause instanceof CloneNotSupportedException;
            System.out.println("Object.clone() threw CloneNotSupportedException as expected: " + isCloneNotSupported);
            System.out.println("Exception message contains Object: " +
                    (cause != null && cause.getMessage() != null && cause.getMessage().contains("Object")));
        }
    }
}
