/** Tests public field access modifier - public fields can be accessed directly from anywhere */
public class Test {
    public static void main(String[] args) {
        // Test public field access
        FieldTestClass obj = new FieldTestClass();

        // Public field can be accessed directly
        obj.publicField = "Modified public field";
        System.out.println("Public field value: " + obj.publicField);

        System.out.println("Public field test passed");
    }
}

class FieldTestClass {
    public String publicField = "Initial public field value";

    public void displayField() {
        System.out.println("Field value: " + publicField);
    }
}
