public class Test {

    record Person(String name, int age) {}

    record Point(int x, int y) {}

    record Book(String title, String author, int pages) {}

    public static void main(String[] args) {
        System.out.println("=== Methods Tests ===");

        testToString();
        testEquals();
        testHashCode();
        testEqualsAndHashCodeConsistency();
    }

    public static void testToString() {
        System.out.println("--- Test toString() ---");
        Person person = new Person("Alice", 30);
        Point point = new Point(10, 20);
        Book book = new Book("1984", "George Orwell", 328);

        System.out.println("Person toString: " + person.toString());
        System.out.println("Point toString: " + point.toString());
        System.out.println("Book toString: " + book.toString());
    }

    public static void testEquals() {
        System.out.println("--- Test equals() ---");
        Person person1 = new Person("Bob", 25);
        Person person2 = new Person("Bob", 25);
        Person person3 = new Person("Charlie", 25);
        Person person4 = new Person("Bob", 30);

        System.out.println("person1.equals(person2): " + person1.equals(person2));
        System.out.println("person1.equals(person3): " + person1.equals(person3));
        System.out.println("person1.equals(person4): " + person1.equals(person4));
        System.out.println("person1.equals(null): " + person1.equals(null));
        System.out.println("person1.equals(\"string\"): " + person1.equals("string"));

        Point point1 = new Point(5, 10);
        Point point2 = new Point(5, 10);
        Point point3 = new Point(10, 5);

        System.out.println("point1.equals(point2): " + point1.equals(point2));
        System.out.println("point1.equals(point3): " + point1.equals(point3));
    }

    public static void testHashCode() {
        System.out.println("--- Test hashCode() ---");
        Person person1 = new Person("Diana", 28);
        Person person2 = new Person("Diana", 28);
        Person person3 = new Person("Eve", 28);

        System.out.println("person1.hashCode(): " + person1.hashCode());
        System.out.println("person2.hashCode(): " + person2.hashCode());
        System.out.println("person3.hashCode(): " + person3.hashCode());
        System.out.println("person1.hashCode() == person2.hashCode(): " + (person1.hashCode() == person2.hashCode()));

        Point point1 = new Point(3, 7);
        Point point2 = new Point(3, 7);
        Point point3 = new Point(7, 3);

        System.out.println("point1.hashCode(): " + point1.hashCode());
        System.out.println("point2.hashCode(): " + point2.hashCode());
        System.out.println("point3.hashCode(): " + point3.hashCode());
        System.out.println("point1.hashCode() == point2.hashCode(): " + (point1.hashCode() == point2.hashCode()));
    }

    public static void testEqualsAndHashCodeConsistency() {
        System.out.println("--- Test equals() and hashCode() Consistency ---");
        Book book1 = new Book("To Kill a Mockingbird", "Harper Lee", 281);
        Book book2 = new Book("To Kill a Mockingbird", "Harper Lee", 281);
        Book book3 = new Book("Pride and Prejudice", "Jane Austen", 432);

        System.out.println("book1.equals(book2): " + book1.equals(book2));
        System.out.println("book1.hashCode() == book2.hashCode(): " + (book1.hashCode() == book2.hashCode()));
        System.out.println("book1.equals(book3): " + book1.equals(book3));
        System.out.println("book1.hashCode() == book3.hashCode(): " + (book1.hashCode() == book3.hashCode()));

        // Test reflexivity
        System.out.println("book1.equals(book1): " + book1.equals(book1));

        // Test symmetry
        System.out.println("book1.equals(book2) == book2.equals(book1): " +
                         (book1.equals(book2) == book2.equals(book1)));
    }
}

