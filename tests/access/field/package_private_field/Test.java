/** Tests package-private field access modifier - fields without access modifier can only be accessed within the same package */
public class Test {
    public static void main(String[] args) {
        // Test package-private field access
        FieldTestClass obj = new FieldTestClass();

        // Package-private field can be accessed within same package
        obj.packagePrivateField = "Modified package-private field";
        System.out.println("Package-private field value: " + obj.packagePrivateField);

        SamePackageClass samePackage = new SamePackageClass();
        samePackage.accessPackagePrivateField(obj);

        System.out.println("Package-private field test passed");
    }
}

class FieldTestClass {
    String packagePrivateField = "Initial package-private field value"; // no access modifier = package-private

    public void displayField() {
        System.out.println("Field value: " + packagePrivateField);
    }
}

class SamePackageClass {
    public void accessPackagePrivateField(FieldTestClass obj) {
        // Package-private field can be accessed from same package
        obj.packagePrivateField = "Modified from another class in same package";
        System.out.println("Package-private field accessed from same package: " + obj.packagePrivateField);
    }
}
