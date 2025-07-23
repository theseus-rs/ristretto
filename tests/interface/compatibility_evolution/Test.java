/** Test interface compatibility and evolution scenarios */
interface OriginalInterface {
    void originalMethod();
    String ORIGINAL_CONSTANT = "original";
}

interface EvolvedInterface extends OriginalInterface {
    // Adding new methods (binary compatible)
    void newMethod();

    // Adding default methods (binary compatible)
    default void newDefaultMethod() {
        System.out.println("EvolvedInterface.newDefaultMethod");
    }

    // Adding static methods (binary compatible)
    static void newStaticMethod() {
        System.out.println("EvolvedInterface.newStaticMethod");
    }

    // Adding new constants (binary compatible)
    String NEW_CONSTANT = "evolved";

    // Overriding default behavior (source compatible)
    default void enhancedBehavior() {
        System.out.println("Enhanced behavior in evolved interface");
        originalMethod();
    }
}

class OriginalImplementation implements OriginalInterface {
    public void originalMethod() {
        System.out.println("OriginalImplementation.originalMethod");
    }
}

class EvolvedImplementation implements EvolvedInterface {
    public void originalMethod() {
        System.out.println("EvolvedImplementation.originalMethod");
    }

    public void newMethod() {
        System.out.println("EvolvedImplementation.newMethod");
    }

    @Override
    public void newDefaultMethod() {
        System.out.println("EvolvedImplementation.newDefaultMethod - overridden");
    }
}

// Test backward compatibility
class BackwardCompatible {
    public static void useOriginalInterface(OriginalInterface original) {
        System.out.println("Using as OriginalInterface:");
        original.originalMethod();
        System.out.println("Original constant: " + OriginalInterface.ORIGINAL_CONSTANT);
    }

    public static void useEvolvedInterface(EvolvedInterface evolved) {
        System.out.println("Using as EvolvedInterface:");
        evolved.originalMethod();
        evolved.newMethod();
        evolved.newDefaultMethod();
        evolved.enhancedBehavior();
        System.out.println("New constant: " + EvolvedInterface.NEW_CONSTANT);
    }
}

// Test with generics evolution
interface GenericV1<T> {
    void process(T item);
}

interface GenericV2<T> extends GenericV1<T> {
    // Adding bounded type parameter method
    <U extends T> U processWithBound(U item);

    // Adding wildcard method
    void processWildcard(GenericV1<? extends T> processor);
}

class GenericEvolution implements GenericV2<String> {
    public void process(String item) {
        System.out.println("Processing string: " + item);
    }

    public <U extends String> U processWithBound(U item) {
        System.out.println("Processing with bound: " + item);
        return item;
    }

    public void processWildcard(GenericV1<? extends String> processor) {
        System.out.println("Processing with wildcard processor");
        // Create a simple string that matches the wildcard constraint
        String testString = "wildcard test";
        // Since ? extends String, we know String is compatible
        @SuppressWarnings("unchecked")
        GenericV1<String> stringProcessor = (GenericV1<String>) processor;
        stringProcessor.process(testString);
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Compatibility and Evolution Test ===");

        // Test original implementation
        OriginalImplementation original = new OriginalImplementation();
        BackwardCompatible.useOriginalInterface(original);

        // Test evolved implementation
        EvolvedImplementation evolved = new EvolvedImplementation();
        BackwardCompatible.useOriginalInterface(evolved); // Should work (backward compatibility)
        BackwardCompatible.useEvolvedInterface(evolved);

        // Test static method from evolved interface
        EvolvedInterface.newStaticMethod();

        // Test interface hierarchy compatibility
        System.out.println("\nInterface hierarchy tests:");
        System.out.println("evolved instanceof OriginalInterface: " + (evolved instanceof OriginalInterface));
        System.out.println("evolved instanceof EvolvedInterface: " + (evolved instanceof EvolvedInterface));
        System.out.println("original instanceof EvolvedInterface: " + (original instanceof EvolvedInterface));

        // Test reference assignment compatibility
        OriginalInterface origRef = evolved; // Evolved can be used as Original
        origRef.originalMethod();

        EvolvedInterface evolvedRef = evolved;
        evolvedRef.originalMethod();
        evolvedRef.newMethod();

        // Test constant access
        System.out.println("\nConstant access tests:");
        System.out.println("Via OriginalInterface: " + OriginalInterface.ORIGINAL_CONSTANT);
        System.out.println("Via EvolvedInterface: " + EvolvedInterface.ORIGINAL_CONSTANT);
        System.out.println("New constant: " + EvolvedInterface.NEW_CONSTANT);

        // Test generic evolution
        GenericEvolution genericEvol = new GenericEvolution();
        genericEvol.process("test");
        String result = genericEvol.processWithBound("bounded");
        System.out.println("Bounded result: " + result);

        // Test wildcard compatibility
        GenericV1<String> simpleProcessor = item -> System.out.println("Simple processor: " + item);
        genericEvol.processWildcard(simpleProcessor);

        // Test generic interface references
        GenericV1<String> v1Ref = genericEvol;
        GenericV2<String> v2Ref = genericEvol;

        v1Ref.process("v1 reference");
        v2Ref.process("v2 reference");
        v2Ref.processWithBound("v2 bounded");

        // Test array compatibility
        OriginalInterface[] origArray = new OriginalInterface[2];
        origArray[0] = original;
        origArray[1] = evolved; // Evolved can go into Original array

        System.out.println("\nArray compatibility test:");
        for (OriginalInterface iface : origArray) {
            iface.originalMethod();
        }

        EvolvedInterface[] evolvedArray = {evolved}; // Only evolved implementations
        for (EvolvedInterface iface : evolvedArray) {
            iface.originalMethod();
            iface.newMethod();
        }

        System.out.println("Interface compatibility tests completed");
    }
}
