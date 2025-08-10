public class Test {

    // Record with custom constructor
    record Person(String name, int age) {
        public Person {
            if (name == null || name.isEmpty()) {
                throw new IllegalArgumentException("Name cannot be null or empty");
            }
            if (age < 0) {
                throw new IllegalArgumentException("Age cannot be negative");
            }
        }
    }

    // Record with custom canonical constructor
    record Point(int x, int y) {
        public Point(int x, int y) {
            this.x = Math.max(0, x);
            this.y = Math.max(0, y);
        }
    }

    // Record with additional constructors
    record Rectangle(int width, int height) {
        public Rectangle {
            if (width <= 0 || height <= 0) {
                throw new IllegalArgumentException("Width and height must be positive");
            }
        }

        // Additional constructor for square
        public Rectangle(int size) {
            this(size, size);
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Constructor Tests ===");

        testCompactConstructor();
        testCanonicalConstructor();
        testAdditionalConstructors();
        testConstructorValidation();
    }

    public static void testCompactConstructor() {
        System.out.println("--- Test Compact Constructor ---");
        try {
            Person person = new Person("Alice", 30);
            System.out.println("Valid person created: " + person);
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }

    public static void testCanonicalConstructor() {
        System.out.println("--- Test Canonical Constructor ---");
        Point point1 = new Point(-5, 10);
        Point point2 = new Point(15, -3);
        System.out.println("Point1 (negative x adjusted): " + point1);
        System.out.println("Point2 (negative y adjusted): " + point2);
    }

    public static void testAdditionalConstructors() {
        System.out.println("--- Test Additional Constructors ---");
        Rectangle rect1 = new Rectangle(10, 20);
        Rectangle square = new Rectangle(15);
        System.out.println("Rectangle: " + rect1);
        System.out.println("Square: " + square);
    }

    public static void testConstructorValidation() {
        System.out.println("--- Test Constructor Validation ---");

        // Test invalid name
        try {
            Person invalidPerson = new Person("", 25);
            System.out.println("Should not reach here");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught expected exception for empty name: " + e.getMessage());
        }

        // Test invalid age
        try {
            Person invalidPerson = new Person("Bob", -5);
            System.out.println("Should not reach here");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught expected exception for negative age: " + e.getMessage());
        }

        // Test invalid rectangle
        try {
            Rectangle invalidRect = new Rectangle(0, 10);
            System.out.println("Should not reach here");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught expected exception for invalid rectangle: " + e.getMessage());
        }
    }
}

