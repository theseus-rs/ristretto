import java.lang.reflect.*;
import java.util.Arrays;

public class Test {

    record Person(String name, int age) {}

    record Point(int x, int y) {}

    record Book(String title, String author, int pages) {}

    public static void main(String[] args) {
        System.out.println("=== Reflection Tests ===");

        testRecordComponents();
        testRecordMethods();
        testRecordFields();
        testRecordClass();
        testRecordConstructors();
    }

    public static void testRecordComponents() {
        System.out.println("--- Test Record Components ---");
        Class<Person> personClass = Person.class;

        System.out.println("Is record: " + personClass.isRecord());
        RecordComponent[] components = personClass.getRecordComponents();
        System.out.println("Number of components: " + components.length);

        for (RecordComponent component : components) {
            System.out.println("Component name: " + component.getName());
            System.out.println("Component type: " + component.getType().getName());
            System.out.println("Component accessor: " + component.getAccessor().getName());
        }
    }

    public static void testRecordMethods() {
        System.out.println("--- Test Record Methods ---");
        Class<Point> pointClass = Point.class;
        Method[] methods = pointClass.getDeclaredMethods();
        Arrays.sort(methods, (a, b) -> a.getName().compareTo(b.getName()));

        System.out.println("Number of methods: " + methods.length);
        for (Method method : methods) {
            System.out.println("Method: " + method.getName() +
                             " - Return type: " + method.getReturnType().getSimpleName() +
                             " - Parameters: " + method.getParameterCount());
        }
    }

    public static void testRecordFields() {
        System.out.println("--- Test Record Fields ---");
        Class<Book> bookClass = Book.class;
        Field[] fields = bookClass.getDeclaredFields();

        System.out.println("Number of fields: " + fields.length);
        for (Field field : fields) {
            System.out.println("Field: " + field.getName() +
                             " - Type: " + field.getType().getSimpleName() +
                             " - Modifiers: " + Modifier.toString(field.getModifiers()));
        }
    }

    public static void testRecordClass() {
        System.out.println("--- Test Record Class Properties ---");
        Class<Person> personClass = Person.class;

        System.out.println("Class name: " + personClass.getName());
        System.out.println("Simple name: " + personClass.getSimpleName());
        System.out.println("Is record: " + personClass.isRecord());
        System.out.println("Is final: " + Modifier.isFinal(personClass.getModifiers()));
        System.out.println("Superclass: " + personClass.getSuperclass().getName());

        Class<?>[] interfaces = personClass.getInterfaces();
        System.out.println("Number of interfaces: " + interfaces.length);
        for (Class<?> iface : interfaces) {
            System.out.println("Interface: " + iface.getName());
        }
    }

    public static void testRecordConstructors() {
        System.out.println("--- Test Record Constructors ---");
        Class<Point> pointClass = Point.class;
        Constructor<?>[] constructors = pointClass.getDeclaredConstructors();

        System.out.println("Number of constructors: " + constructors.length);
        for (Constructor<?> constructor : constructors) {
            System.out.println("Constructor parameters: " + constructor.getParameterCount());
            Parameter[] parameters = constructor.getParameters();
            for (Parameter param : parameters) {
                System.out.println("  Parameter: " + param.getName() + " - Type: " + param.getType().getSimpleName());
            }
        }

        // Test creating instance via reflection
        try {
            Constructor<Point> constructor = pointClass.getDeclaredConstructor(int.class, int.class);
            Point reflectedPoint = constructor.newInstance(100, 200);
            System.out.println("Created via reflection: " + reflectedPoint);
        } catch (Exception e) {
            System.out.println("Reflection instantiation failed: " + e.getMessage());
        }
    }
}
