public class Test {

    // Nested record inside class
    record Address(String street, String city, String state, int zipCode) {}

    record Person(String name, int age, Address address) {}

    // Static nested record
    static record Company(String name, Address headquarters) {}

    public static void main(String[] args) {
        System.out.println("=== Nested Records Tests ===");

        testNestedRecords();
        testLocalRecords();
        testRecordWithNestedRecord();
        testStaticNestedRecord();
        testNestedRecordAccess();
    }

    public static void testNestedRecords() {
        System.out.println("--- Test Nested Records ---");
        Address address = new Address("123 Main St", "Springfield", "IL", 62701);
        Person person = new Person("Alice", 30, address);

        System.out.println("Person: " + person);
        System.out.println("Person name: " + person.name());
        System.out.println("Person address: " + person.address());
        System.out.println("Person street: " + person.address().street());
        System.out.println("Person city: " + person.address().city());
    }

    public static void testLocalRecords() {
        System.out.println("--- Test Local Records ---");

        // Local record inside method
        record Point(int x, int y) {
            public double distanceFromOrigin() {
                return Math.sqrt(x * x + y * y);
            }
        }

        Point point1 = new Point(3, 4);
        Point point2 = new Point(5, 12);

        System.out.println("Point1: " + point1);
        System.out.println("Point1 distance from origin: " + point1.distanceFromOrigin());
        System.out.println("Point2: " + point2);
        System.out.println("Point2 distance from origin: " + point2.distanceFromOrigin());
    }

    public static void testRecordWithNestedRecord() {
        System.out.println("--- Test Record With Nested Record ---");

        // Record containing another record
        record Department(String name, Person manager) {}

        Address managerAddress = new Address("456 Oak Ave", "Chicago", "IL", 60601);
        Person manager = new Person("Bob", 45, managerAddress);
        Department department = new Department("Engineering", manager);

        System.out.println("Department: " + department);
        System.out.println("Department name: " + department.name());
        System.out.println("Manager: " + department.manager());
        System.out.println("Manager name: " + department.manager().name());
        System.out.println("Manager address: " + department.manager().address());
    }

    public static void testStaticNestedRecord() {
        System.out.println("--- Test Static Nested Record ---");
        Address headquarters = new Address("789 Business Blvd", "New York", "NY", 10001);
        Company company = new Company("Tech Corp", headquarters);

        System.out.println("Company: " + company);
        System.out.println("Company name: " + company.name());
        System.out.println("Headquarters: " + company.headquarters());
        System.out.println("Headquarters city: " + company.headquarters().city());
    }

    public static void testNestedRecordAccess() {
        System.out.println("--- Test Nested Record Access ---");

        // Test accessing nested components
        Address address = new Address("321 Elm St", "Boston", "MA", 2101);
        Person person = new Person("Charlie", 35, address);

        // Direct access
        System.out.println("Direct access - name: " + person.name());
        System.out.println("Direct access - age: " + person.age());
        System.out.println("Direct access - full address: " + person.address());

        // Chained access
        System.out.println("Chained access - street: " + person.address().street());
        System.out.println("Chained access - city: " + person.address().city());
        System.out.println("Chained access - state: " + person.address().state());
        System.out.println("Chained access - zip: " + person.address().zipCode());

        // Test equality with nested records
        Address address2 = new Address("321 Elm St", "Boston", "MA", 2101);
        Person person2 = new Person("Charlie", 35, address2);

        System.out.println("Addresses equal: " + address.equals(address2));
        System.out.println("Persons equal: " + person.equals(person2));
    }

    // Inner class containing records
    static class Container {
        record Item(String name, double price) {}

        static void demonstrateInnerRecord() {
            System.out.println("--- Test Record in Inner Class ---");
            Item item = new Item("Widget", 19.99);
            System.out.println("Item: " + item);
            System.out.println("Item name: " + item.name());
            System.out.println("Item price: " + item.price());
        }
    }

    static {
        Container.demonstrateInnerRecord();
    }
}

