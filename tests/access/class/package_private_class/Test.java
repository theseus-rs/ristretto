/** Tests package-private class access modifier - classes without access modifier can only be accessed within the same package */
public class Test {
    public static void main(String[] args) {
        // Test package-private class access
        PackagePrivateClass pkg = new PackagePrivateClass();
        pkg.testMethod();
        System.out.println("Package-private class test passed");
    }
}

// Package-private class (no access modifier) can only be accessed within the same package
class PackagePrivateClass {
    public void testMethod() {
        System.out.println("Package-private class method called");
    }
}
