/** Tests protected method access modifier - protected methods can be called by subclasses and classes in the same package */
public class Test {
    public static void main(String[] args) {
        // Test protected method access
        SubClass obj = new SubClass();
        obj.testProtectedAccess();

        SamePackageClass samePackage = new SamePackageClass();
        samePackage.testProtectedFromSamePackage();

        System.out.println("Protected method test passed");
    }
}

class BaseClass {
    protected void protectedMethod() {
        System.out.println("Protected method called");
    }

    protected static void staticProtectedMethod() {
        System.out.println("Static protected method called");
    }

    protected String protectedMethodWithReturn() {
        return "Protected method return value";
    }
}

class SubClass extends BaseClass {
    public void testProtectedAccess() {
        // Protected method can be called by subclasses
        protectedMethod();
        staticProtectedMethod();
        String result = protectedMethodWithReturn();
        System.out.println("Protected method result: " + result);
        System.out.println("Protected methods called from subclass");
    }
}

class SamePackageClass {
    public void testProtectedFromSamePackage() {
        BaseClass base = new BaseClass();
        // Protected method can be called from same package
        base.protectedMethod();
        BaseClass.staticProtectedMethod();
        System.out.println("Protected methods called from same package");
    }
}
