/** Tests private field access modifier - private fields can only be accessed within the same class */
public class Test {
    public static void main(String[] args) {
        // Test private field access
        FieldTestClass obj = new FieldTestClass();

        // Private field cannot be accessed directly from outside the class
        // obj.privateField = "This would cause a compile error";

        // Access private field through public methods
        obj.setPrivateField("Modified private field");
        System.out.println("Private field value: " + obj.getPrivateField());

        System.out.println("Private field test passed");
    }
}

class FieldTestClass {
    private String privateField = "Initial private field value";

    public String getPrivateField() {
        return privateField;
    }

    public void setPrivateField(String value) {
        this.privateField = value;
    }

    public void testPrivateFieldAccess() {
        // Private field can be accessed within the same class
        privateField = "Accessed from same class";
        System.out.println("Private field accessed from same class: " + privateField);
    }
}
