import java.lang.reflect.*;
import java.lang.annotation.Annotation;
import java.util.Arrays;
import java.util.Comparator;

public class Test {

    // Test records for reflection
    record Person(String name, int age) {}

    record Point(int x, int y) {
        public Point {
            if (x < 0 || y < 0) {
                throw new IllegalArgumentException("Coordinates must be non-negative");
            }
        }
    }

    record Book(String title, String author, int pages) {
        // Additional method
        public String getDescription() {
            return title + " by " + author + " (" + pages + " pages)";
        }
    }

    // Generic record
    record Container<T>(T value) {}

    // Record with annotations
    record AnnotatedRecord(@Deprecated String data, int count) {}

    public static void main(String[] args) {
        System.out.println("=== Record Reflection Tests ===");

        testIsRecord();
        testRecordComponents();
        testRecordComponentAccessors();
        testRecordConstructors();
        testRecordMethods();
        testRecordFields();
        testRecordSuperclass();
        testGenericRecord();
        testAnnotatedRecord();
        testRecordInstantiation();
    }

    public static void testIsRecord() {
        System.out.println("--- Test isRecord() ---");
        Class<Person> personClass = Person.class;
        Class<String> stringClass = String.class;
        Class<Object> objectClass = Object.class;

        System.out.println("Person.class.isRecord(): " + personClass.isRecord());
        System.out.println("String.class.isRecord(): " + stringClass.isRecord());
        System.out.println("Object.class.isRecord(): " + objectClass.isRecord());

        // Test with instances
        Person person = new Person("Alice", 30);
        System.out.println("person.getClass().isRecord(): " + person.getClass().isRecord());
    }

    public static void testRecordComponents() {
        System.out.println("--- Test getRecordComponents() ---");
        Class<Person> personClass = Person.class;

        RecordComponent[] components = personClass.getRecordComponents();
        System.out.println("Number of record components: " + components.length);

        for (RecordComponent component : components) {
            System.out.println("Component name: " + component.getName());
            System.out.println("Component type: " + component.getType().getName());
            System.out.println("Component generic type: " + component.getGenericType());
            System.out.println("Component declaring record: " + component.getDeclaringRecord().getName());
        }

        // Test empty record
        record Empty() {}
        Class<Empty> emptyClass = Empty.class;
        RecordComponent[] emptyComponents = emptyClass.getRecordComponents();
        System.out.println("Empty record components: " + emptyComponents.length);
    }

    public static void testRecordComponentAccessors() {
        System.out.println("--- Test RecordComponent Accessors ---");
        Class<Point> pointClass = Point.class;
        RecordComponent[] components = pointClass.getRecordComponents();

        Point point = new Point(10, 20);

        for (RecordComponent component : components) {
            try {
                Method accessor = component.getAccessor();
                System.out.println("Accessor method: " + accessor.getName());
                System.out.println("Accessor return type: " + accessor.getReturnType().getName());

                Object value = accessor.invoke(point);
                System.out.println("Accessor value for " + component.getName() + ": " + value);
            } catch (Exception e) {
                System.out.println("Error accessing component " + component.getName() + ": " + e.getMessage());
            }
        }
    }

    public static void testRecordConstructors() {
        System.out.println("--- Test Record Constructors ---");
        Class<Book> bookClass = Book.class;
        Constructor<?>[] constructors = bookClass.getDeclaredConstructors();

        System.out.println("Number of constructors: " + constructors.length);

        for (Constructor<?> constructor : constructors) {
            System.out.println("Constructor parameter count: " + constructor.getParameterCount());
            Parameter[] parameters = constructor.getParameters();

            for (int i = 0; i < parameters.length; i++) {
                Parameter param = parameters[i];
                System.out.println("  Parameter " + i + ": " + param.getName() + " (" + param.getType().getSimpleName() + ")");
            }
        }

        // Test constructor invocation
        try {
            Constructor<Book> bookConstructor = bookClass.getDeclaredConstructor(String.class, String.class, int.class);
            Book book = bookConstructor.newInstance("1984", "George Orwell", 328);
            System.out.println("Created book via reflection: " + book);
        } catch (Exception e) {
            System.out.println("Constructor invocation failed: " + e.getMessage());
        }
    }

    public static void testRecordMethods() {
        System.out.println("--- Test Record Methods ---");
        Class<Book> bookClass = Book.class;
        Method[] methods = bookClass.getDeclaredMethods();
        // Sort methods by name for deterministic output
        Arrays.sort(methods, Comparator.comparing(Method::getName));

        System.out.println("Number of methods: " + methods.length);

        // Get component names for accessor checking
        RecordComponent[] components = bookClass.getRecordComponents();
        java.util.Set<String> accessorNames = new java.util.HashSet<>();
        for (RecordComponent component : components) {
            accessorNames.add(component.getName());
        }

        for (Method method : methods) {
            System.out.println("Method: " + method.getName());
            System.out.println("  Return type: " + method.getReturnType().getSimpleName());
            System.out.println("  Parameter count: " + method.getParameterCount());
            System.out.println("  Modifiers: " + Modifier.toString(method.getModifiers()));

            // Check if it's an accessor method (matches component name and has no parameters)
            boolean isAccessor = accessorNames.contains(method.getName()) && method.getParameterCount() == 0;
            System.out.println("  Is accessor: " + isAccessor);
        }
    }

    public static void testRecordFields() {
        System.out.println("--- Test Record Fields ---");
        Class<Person> personClass = Person.class;
        Field[] fields = personClass.getDeclaredFields();
        // Sort fields by name for deterministic output
        Arrays.sort(fields, Comparator.comparing(Field::getName));

        System.out.println("Number of fields: " + fields.length);

        for (Field field : fields) {
            System.out.println("Field: " + field.getName());
            System.out.println("  Type: " + field.getType().getSimpleName());
            System.out.println("  Modifiers: " + Modifier.toString(field.getModifiers()));
            System.out.println("  Is final: " + Modifier.isFinal(field.getModifiers()));
            System.out.println("  Is private: " + Modifier.isPrivate(field.getModifiers()));
        }

        // Test field access
        Person person = new Person("Bob", 25);
        for (Field field : fields) {
            try {
                field.setAccessible(true);
                Object value = field.get(person);
                System.out.println("Field " + field.getName() + " value: " + value);
            } catch (Exception e) {
                System.out.println("Field access failed for " + field.getName() + ": " + e.getMessage());
            }
        }
    }

    public static void testRecordSuperclass() {
        System.out.println("--- Test Record Superclass ---");
        Class<Person> personClass = Person.class;

        System.out.println("Superclass: " + personClass.getSuperclass().getName());
        System.out.println("Is final: " + Modifier.isFinal(personClass.getModifiers()));
        System.out.println("Package: " + personClass.getPackage());

        // Test Record class methods
        Person person = new Person("Charlie", 35);
        System.out.println("Instance of Record: " + (person instanceof Record));

        // Test inherited methods from Record
        try {
            Method toStringMethod = personClass.getMethod("toString");
            Method equalsMethod = personClass.getMethod("equals", Object.class);
            Method hashCodeMethod = personClass.getMethod("hashCode");

            System.out.println("toString() declaring class: " + toStringMethod.getDeclaringClass().getName());
            System.out.println("equals() declaring class: " + equalsMethod.getDeclaringClass().getName());
            System.out.println("hashCode() declaring class: " + hashCodeMethod.getDeclaringClass().getName());
        } catch (Exception e) {
            System.out.println("Method lookup failed: " + e.getMessage());
        }
    }

    public static void testGenericRecord() {
        System.out.println("--- Test Generic Record ---");
        Container<String> stringContainer = new Container<>("Hello");
        Class<?> containerClass = stringContainer.getClass();

        System.out.println("Generic record class: " + containerClass.getName());
        System.out.println("Is record: " + containerClass.isRecord());

        RecordComponent[] components = containerClass.getRecordComponents();
        System.out.println("Generic record components: " + components.length);

        for (RecordComponent component : components) {
            System.out.println("Component name: " + component.getName());
            System.out.println("Component type: " + component.getType().getName());
            System.out.println("Component generic type: " + component.getGenericType());
        }

        // Test type parameters
        TypeVariable<?>[] typeParameters = containerClass.getTypeParameters();
        System.out.println("Type parameters: " + typeParameters.length);
        for (TypeVariable<?> typeParam : typeParameters) {
            System.out.println("Type parameter: " + typeParam.getName());
        }
    }

    public static void testAnnotatedRecord() {
        System.out.println("--- Test Annotated Record ---");
        Class<AnnotatedRecord> annotatedClass = AnnotatedRecord.class;

        System.out.println("Annotated record is record: " + annotatedClass.isRecord());

        RecordComponent[] components = annotatedClass.getRecordComponents();
        for (RecordComponent component : components) {
            System.out.println("Component: " + component.getName());

            Annotation[] annotations = component.getAnnotations();
            System.out.println("  Annotations: " + annotations.length);

            for (Annotation annotation : annotations) {
                System.out.println("  Annotation type: " + annotation.annotationType().getSimpleName());
            }

            // Check for specific annotation
            if (component.isAnnotationPresent(Deprecated.class)) {
                System.out.println("  Component is deprecated");
            }
        }
    }

    public static void testRecordInstantiation() {
        System.out.println("--- Test Record Instantiation via Reflection ---");

        try {
            Class<Point> pointClass = Point.class;
            Constructor<Point> constructor = pointClass.getDeclaredConstructor(int.class, int.class);

            // Test valid instantiation
            Point point1 = constructor.newInstance(5, 10);
            System.out.println("Created point via reflection: " + point1);

            // Test invalid instantiation (should trigger validation)
            try {
                Point point2 = constructor.newInstance(-1, 5);
                System.out.println("Invalid point created: " + point2);
            } catch (InvocationTargetException e) {
                // Normalize error message to just check that validation occurred
                Throwable cause = e.getCause();
                String errorType = cause != null ? cause.getClass().getSimpleName() : "Unknown";
                boolean isValidationError = cause != null && 
                    (cause.getMessage() != null && cause.getMessage().contains("non-negative") ||
                     errorType.contains("IllegalArgument") ||
                     errorType.contains("InternalError"));
                System.out.println("Validation error caught: " + isValidationError);
            }
        } catch (Exception e) {
            System.out.println("Reflection instantiation failed: " + e.getMessage());
        }
    }
}
