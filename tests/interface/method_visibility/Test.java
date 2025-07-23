/** Test interface method visibility and access modifiers */
interface PublicInterface {
    // All interface methods are implicitly public
    void implicitlyPublic();
    public void explicitlyPublic();

    // Default methods are also public
    default void defaultMethod() {
        System.out.println("PublicInterface.defaultMethod");
    }

    // Static methods are public
    static void staticMethod() {
        System.out.println("PublicInterface.staticMethod");
    }
}

// Package-private interface
interface PackageInterface {
    void packageMethod();
}

class VisibilityTest implements PublicInterface, PackageInterface {
    // Must be public since interface methods are public
    public void implicitlyPublic() {
        System.out.println("VisibilityTest.implicitlyPublic");
    }

    public void explicitlyPublic() {
        System.out.println("VisibilityTest.explicitlyPublic");
    }

    // Package method implementation
    public void packageMethod() {
        System.out.println("VisibilityTest.packageMethod");
    }

    // Private method - not from interface
    private void privateMethod() {
        System.out.println("VisibilityTest.privateMethod");
    }

    // Protected method - not from interface
    protected void protectedMethod() {
        System.out.println("VisibilityTest.protectedMethod");
    }
}

// Test with inheritance and visibility
class SubVisibilityTest extends VisibilityTest {
    @Override
    public void implicitlyPublic() {
        System.out.println("SubVisibilityTest.implicitlyPublic");
        super.implicitlyPublic();
    }

    @Override
    protected void protectedMethod() {
        System.out.println("SubVisibilityTest.protectedMethod");
        super.protectedMethod();
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Method Visibility Test ===");

        VisibilityTest test = new VisibilityTest();
        test.implicitlyPublic();
        test.explicitlyPublic();
        test.defaultMethod();
        test.packageMethod();
        test.protectedMethod();

        // Static method call
        PublicInterface.staticMethod();

        // Test via interface references
        PublicInterface pubRef = test;
        pubRef.implicitlyPublic();
        pubRef.explicitlyPublic();
        pubRef.defaultMethod();

        PackageInterface pkgRef = test;
        pkgRef.packageMethod();

        // Test inheritance
        SubVisibilityTest subTest = new SubVisibilityTest();
        subTest.implicitlyPublic();
        subTest.explicitlyPublic();
        subTest.protectedMethod();

        // Test instanceof
        System.out.println("test instanceof PublicInterface: " + (test instanceof PublicInterface));
        System.out.println("test instanceof PackageInterface: " + (test instanceof PackageInterface));
        System.out.println("subTest instanceof PublicInterface: " + (subTest instanceof PublicInterface));
        System.out.println("subTest instanceof VisibilityTest: " + (subTest instanceof VisibilityTest));

        System.out.println("All visibility tests completed");
    }
}

