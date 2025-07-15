/** Tests package-private method access modifier - methods without access modifier can only be called within the same package */
public class Test {
    public static void main(String[] args) {
        // Test package-private method access
        MethodTestClass obj = new MethodTestClass();

        // Package-private method can be called within same package
        obj.packagePrivateMethod();
        MethodTestClass.staticPackagePrivateMethod();

        SamePackageClass samePackage = new SamePackageClass();
        samePackage.accessPackagePrivateMethod(obj);

        System.out.println("Package-private method test passed");
    }
}

class MethodTestClass {
    void packagePrivateMethod() { // no access modifier = package-private
        System.out.println("Package-private method called");
    }

    static void staticPackagePrivateMethod() {
        System.out.println("Static package-private method called");
    }

    String packagePrivateMethodWithReturn() {
        return "Package-private method return value";
    }
}

class SamePackageClass {
    public void accessPackagePrivateMethod(MethodTestClass obj) {
        // Package-private method can be called from same package
        obj.packagePrivateMethod();
        MethodTestClass.staticPackagePrivateMethod();
        String result = obj.packagePrivateMethodWithReturn();
        System.out.println("Package-private method result: " + result);
        System.out.println("Package-private methods called from another class in same package");
    }
}
