public class Test {

    // Basic record definition
    record Person(String name, int age) {}

    record Point(int x, int y) {}

    record Empty() {}

    public static void main(String[] args) {
        System.out.println("=== Basic Record Tests ===");

        testBasicRecordCreation();
        testRecordAccessors();
        testEmptyRecord();
        testRecordWithMultipleComponents();
        testRecordInstantiation();
    }

    public static void testBasicRecordCreation() {
        System.out.println("--- Test Basic Record Creation ---");
        Person person = new Person("Alice", 30);
        System.out.println("Created person: " + person);
        System.out.println("Person class: " + person.getClass().getName());
        System.out.println("Is record: " + person.getClass().isRecord());
    }

    public static void testRecordAccessors() {
        System.out.println("--- Test Record Accessors ---");
        Person person = new Person("Bob", 25);
        System.out.println("Name: " + person.name());
        System.out.println("Age: " + person.age());

        Point point = new Point(10, 20);
        System.out.println("X: " + point.x());
        System.out.println("Y: " + point.y());
    }

    public static void testEmptyRecord() {
        System.out.println("--- Test Empty Record ---");
        Empty empty = new Empty();
        System.out.println("Empty record: " + empty);
        System.out.println("Empty record class: " + empty.getClass().getName());
        System.out.println("Is record: " + empty.getClass().isRecord());
    }

    public static void testRecordWithMultipleComponents() {
        System.out.println("--- Test Record With Multiple Components ---");
        record Address(String street, String city, String state, int zipCode) {}

        Address address = new Address("123 Main St", "Springfield", "IL", 62701);
        System.out.println("Address: " + address);
        System.out.println("Street: " + address.street());
        System.out.println("City: " + address.city());
        System.out.println("State: " + address.state());
        System.out.println("Zip: " + address.zipCode());
    }

    public static void testRecordInstantiation() {
        System.out.println("--- Test Record Instantiation ---");
        Person[] people = {
            new Person("Charlie", 35),
            new Person("Diana", 28),
            new Person("Eve", 42)
        };

        for (Person p : people) {
            System.out.println("Person: " + p.name() + " is " + p.age() + " years old");
        }
    }
}

