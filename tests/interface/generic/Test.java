/** Test generic interfaces and type parameters */
interface GenericInterface<T> {
    void process(T item);
    T get();
    boolean isValid(T item);

    default void printType(T item) {
        System.out.println("Item type: " + item.getClass().getSimpleName());
        System.out.println("Item value: " + item);
    }
}

interface BoundedGeneric<T extends Number> {
    T calculate(T a, T b);
    default void printNumber(T num) {
        System.out.println("Number: " + num + ", doubleValue: " + num.doubleValue());
    }
}

interface MultipleGenerics<T, U, V> {
    V combine(T first, U second);
    T getFirst();
    U getSecond();
}

class StringProcessor implements GenericInterface<String> {
    private String value = "default";

    public void process(String item) {
        System.out.println("Processing string: " + item);
        this.value = item.toUpperCase();
    }

    public String get() {
        return value;
    }

    public boolean isValid(String item) {
        return item != null && !item.isEmpty();
    }
}

class IntegerProcessor implements GenericInterface<Integer> {
    private Integer value = 0;

    public void process(Integer item) {
        System.out.println("Processing integer: " + item);
        this.value = item * 2;
    }

    public Integer get() {
        return value;
    }

    public boolean isValid(Integer item) {
        return item != null && item >= 0;
    }
}

class NumberCalculator implements BoundedGeneric<Double> {
    public Double calculate(Double a, Double b) {
        System.out.println("Calculating: " + a + " + " + b);
        return a + b;
    }
}

class StringCombiner implements MultipleGenerics<String, Integer, String> {
    public String combine(String first, Integer second) {
        System.out.println("Combining string '" + first + "' with integer " + second);
        return first + second;
    }

    public String getFirst() {
        return "first";
    }

    public Integer getSecond() {
        return 42;
    }
}

// Raw type usage (not recommended but legal)
class RawProcessor implements GenericInterface {
    private Object value = null;

    public void process(Object item) {
        System.out.println("Raw processing: " + item);
        this.value = item;
    }

    public Object get() {
        return value;
    }

    public boolean isValid(Object item) {
        return item != null;
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Generic Interfaces Test ===");

        // Test generic interface with String
        StringProcessor stringProc = new StringProcessor();
        stringProc.process("hello world");
        String result1 = stringProc.get();
        System.out.println("String result: " + result1);
        System.out.println("Is 'test' valid: " + stringProc.isValid("test"));
        System.out.println("Is empty string valid: " + stringProc.isValid(""));
        stringProc.printType("example");

        // Test generic interface with Integer
        IntegerProcessor intProc = new IntegerProcessor();
        intProc.process(21);
        Integer result2 = intProc.get();
        System.out.println("Integer result: " + result2);
        System.out.println("Is 10 valid: " + intProc.isValid(10));
        System.out.println("Is -5 valid: " + intProc.isValid(-5));
        intProc.printType(100);

        // Test bounded generic
        NumberCalculator calc = new NumberCalculator();
        Double calcResult = calc.calculate(3.14, 2.86);
        System.out.println("Calculation result: " + calcResult);
        calc.printNumber(42.5);

        // Test multiple generics
        StringCombiner combiner = new StringCombiner();
        String combined = combiner.combine("Value: ", 123);
        System.out.println("Combined result: " + combined);
        System.out.println("First: " + combiner.getFirst());
        System.out.println("Second: " + combiner.getSecond());

        // Test raw type (generates warnings but works)
        RawProcessor rawProc = new RawProcessor();
        rawProc.process("raw string");
        rawProc.process(42);
        Object rawResult = rawProc.get();
        System.out.println("Raw result: " + rawResult);

        // Test instanceof with generics
        System.out.println("stringProc instanceof GenericInterface: " + (stringProc instanceof GenericInterface));
        System.out.println("intProc instanceof GenericInterface: " + (intProc instanceof GenericInterface));
        System.out.println("calc instanceof BoundedGeneric: " + (calc instanceof BoundedGeneric));

        // Test interface references with generics
        GenericInterface<String> stringRef = stringProc;
        GenericInterface<Integer> intRef = intProc;

        stringRef.process("reference test");
        intRef.process(999);
    }
}

