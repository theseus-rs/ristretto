/** Test interface edge cases and boundary conditions */
interface EmptyInterface {
    // Empty interface - valid but has no methods
}

interface MarkerInterface {
    // Marker interface with only constants
    int MARKER_VALUE = 100;
    String MARKER_NAME = "MarkerInterface";
}

interface OverloadedMethods {
    void method();
    void method(int param);
    void method(String param);
    void method(int param1, String param2);
    int method(double param); // Different return type with different params is OK
}

interface CovariantReturns {
    Object getObject();
    Number getNumber();
    String getString();
}

class EdgeCaseImplementation implements EmptyInterface, MarkerInterface, OverloadedMethods {
    public void method() {
        System.out.println("EdgeCaseImplementation.method()");
    }

    public void method(int param) {
        System.out.println("EdgeCaseImplementation.method(int): " + param);
    }

    public void method(String param) {
        System.out.println("EdgeCaseImplementation.method(String): " + param);
    }

    public void method(int param1, String param2) {
        System.out.println("EdgeCaseImplementation.method(int, String): " + param1 + ", " + param2);
    }

    public int method(double param) {
        System.out.println("EdgeCaseImplementation.method(double): " + param);
        return (int) param;
    }
}

abstract class CovariantBase implements CovariantReturns {
    public Object getObject() {
        return "Base Object";
    }

    public Number getNumber() {
        return 42;
    }

    // Leave getString abstract
    public abstract String getString();
}

class CovariantImplementation extends CovariantBase {
    @Override
    public String getObject() { // Covariant return - String is subtype of Object
        return "Covariant String Object";
    }

    @Override
    public Integer getNumber() { // Covariant return - Integer is subtype of Number
        return 123;
    }

    @Override
    public String getString() {
        return "Implemented String";
    }
}

// Test interface with throws clauses
interface ThrowingInterface {
    void mayThrow() throws Exception;
    void specificThrow() throws IllegalArgumentException;
    void multipleThrows() throws IllegalArgumentException, IllegalStateException;
}

class ThrowingImplementation implements ThrowingInterface {
    public void mayThrow() throws Exception {
        System.out.println("ThrowingImplementation.mayThrow - might throw");
        // Implementation can choose to throw or not
    }

    public void specificThrow() throws IllegalArgumentException {
        System.out.println("ThrowingImplementation.specificThrow");
        // Can throw same or subtype exceptions
    }

    public void multipleThrows() {
        System.out.println("ThrowingImplementation.multipleThrows");
        // Implementation doesn't have to declare throws if it doesn't throw
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Edge Cases Test ===");

        // Test empty and marker interfaces
        EdgeCaseImplementation edge = new EdgeCaseImplementation();
        System.out.println("edge instanceof EmptyInterface: " + (edge instanceof EmptyInterface));
        System.out.println("edge instanceof MarkerInterface: " + (edge instanceof MarkerInterface));
        System.out.println("MARKER_VALUE: " + MarkerInterface.MARKER_VALUE);
        System.out.println("MARKER_NAME: " + MarkerInterface.MARKER_NAME);

        // Test method overloading
        edge.method();
        edge.method(42);
        edge.method("test");
        edge.method(10, "hello");
        int result = edge.method(3.14);
        System.out.println("method(double) returned: " + result);

        // Test covariant returns
        CovariantImplementation covariant = new CovariantImplementation();
        Object obj = covariant.getObject();
        Number num = covariant.getNumber();
        String str = covariant.getString();

        System.out.println("getObject returned: " + obj + " (type: " + obj.getClass().getSimpleName() + ")");
        System.out.println("getNumber returned: " + num + " (type: " + num.getClass().getSimpleName() + ")");
        System.out.println("getString returned: " + str + " (type: " + str.getClass().getSimpleName() + ")");

        // Test via interface reference
        CovariantReturns covRef = covariant;
        Object objRef = covRef.getObject();
        Number numRef = covRef.getNumber();
        String strRef = covRef.getString();

        System.out.println("Via interface - getObject: " + objRef);
        System.out.println("Via interface - getNumber: " + numRef);
        System.out.println("Via interface - getString: " + strRef);

        // Test throwing interface
        ThrowingImplementation throwing = new ThrowingImplementation();
        try {
            throwing.mayThrow();
            throwing.specificThrow();
            throwing.multipleThrows();
            System.out.println("No exceptions thrown");
        } catch (Exception e) {
            System.out.println("Exception caught: " + e.getClass().getSimpleName());
        }

        // Test null assignments
        EmptyInterface nullEmpty = null;
        MarkerInterface nullMarker = null;
        OverloadedMethods nullOverloaded = null;

        System.out.println("null instanceof EmptyInterface: " + (nullEmpty instanceof EmptyInterface));
        System.out.println("null instanceof MarkerInterface: " + (nullMarker instanceof MarkerInterface));
        System.out.println("null instanceof OverloadedMethods: " + (nullOverloaded instanceof OverloadedMethods));

        System.out.println("Edge cases test completed");
    }
}
