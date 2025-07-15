/** Tests protected field access modifier - protected fields can be accessed by subclasses and classes in the same package */
public class Test {
    public static void main(String[] args) {
        // Test protected field access
        SubClass obj = new SubClass();
        obj.testProtectedAccess();

        System.out.println("Protected field test passed");
    }
}

class BaseClass {
    protected String protectedField = "Initial protected field value";
}

class SubClass extends BaseClass {
    public void testProtectedAccess() {
        // Protected field can be accessed by subclasses
        protectedField = "Modified by subclass";
        System.out.println("Protected field value: " + protectedField);

        // Can also access through inheritance
        System.out.println("Accessing inherited protected field: " + this.protectedField);
    }
}

class SamePackageClass {
    public void testProtectedFromSamePackage() {
        BaseClass base = new BaseClass();
        // Protected field can be accessed from same package
        base.protectedField = "Modified from same package";
        System.out.println("Protected field from same package: " + base.protectedField);
    }
}
