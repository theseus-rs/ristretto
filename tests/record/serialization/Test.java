import java.io.*;

public class Test {

    record Person(String name, int age) implements Serializable {}

    record Point(int x, int y) implements Serializable {}

    record Book(String title, String author, int pages) implements Serializable {
        private static final long serialVersionUID = 1L;
    }

    public static void main(String[] args) {
        System.out.println("=== Serialization Tests ===");

        testRecordSerialization();
        testRecordDeserialization();
        testSerializationRoundTrip();
        testSerializableInterface();
    }

    public static void testRecordSerialization() {
        System.out.println("--- Test Record Serialization ---");
        Person person = new Person("Alice", 30);

        try {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(person);
            oos.close();

            byte[] serializedData = baos.toByteArray();
            System.out.println("Person serialized successfully, size: " + serializedData.length + " bytes");
        } catch (IOException e) {
            System.out.println("Serialization failed: " + e.getMessage());
        }
    }

    public static void testRecordDeserialization() {
        System.out.println("--- Test Record Deserialization ---");
        Point originalPoint = new Point(10, 20);

        try {
            // Serialize
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(originalPoint);
            oos.close();

            // Deserialize
            ByteArrayInputStream bais = new ByteArrayInputStream(baos.toByteArray());
            ObjectInputStream ois = new ObjectInputStream(bais);
            Point deserializedPoint = (Point) ois.readObject();
            ois.close();

            System.out.println("Original point: " + originalPoint);
            System.out.println("Deserialized point: " + deserializedPoint);
            System.out.println("Points are equal: " + originalPoint.equals(deserializedPoint));
            System.out.println("Same object reference: " + (originalPoint == deserializedPoint));
        } catch (IOException | ClassNotFoundException e) {
            System.out.println("Deserialization failed: " + e.getMessage());
        }
    }

    public static void testSerializationRoundTrip() {
        System.out.println("--- Test Serialization Round Trip ---");
        Book originalBook = new Book("1984", "George Orwell", 328);

        try {
            // Serialize
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(originalBook);
            oos.close();

            // Deserialize
            ByteArrayInputStream bais = new ByteArrayInputStream(baos.toByteArray());
            ObjectInputStream ois = new ObjectInputStream(bais);
            Book deserializedBook = (Book) ois.readObject();
            ois.close();

            System.out.println("Original: " + originalBook);
            System.out.println("Round-trip: " + deserializedBook);
            System.out.println("Title preserved: " + originalBook.title().equals(deserializedBook.title()));
            System.out.println("Author preserved: " + originalBook.author().equals(deserializedBook.author()));
            System.out.println("Pages preserved: " + (originalBook.pages() == deserializedBook.pages()));
            System.out.println("Hash codes equal: " + (originalBook.hashCode() == deserializedBook.hashCode()));
        } catch (IOException | ClassNotFoundException e) {
            System.out.println("Round-trip failed: " + e.getMessage());
        }
    }

    public static void testSerializableInterface() {
        System.out.println("--- Test Serializable Interface ---");
        Person person = new Person("Bob", 25);
        Point point = new Point(5, 15);
        Book book = new Book("To Kill a Mockingbird", "Harper Lee", 281);

        System.out.println("Person implements Serializable: " + (person instanceof Serializable));
        System.out.println("Point implements Serializable: " + (point instanceof Serializable));
        System.out.println("Book implements Serializable: " + (book instanceof Serializable));
    }
}
