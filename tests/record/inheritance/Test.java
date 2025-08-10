interface Drawable {
    void draw();
}

interface Comparable<T> {
    int compareTo(T other);
}

public class Test {

    // Record implementing interfaces
    record Circle(double radius) implements Drawable {
        public void draw() {
            System.out.println("Drawing circle with radius: " + radius);
        }
    }

    record Person(String name, int age) implements Comparable<Person> {
        public int compareTo(Person other) {
            int nameComparison = this.name.compareTo(other.name);
            if (nameComparison != 0) {
                return nameComparison;
            }
            return Integer.compare(this.age, other.age);
        }
    }

    // Records cannot extend classes (other than Record)
    // This would cause compilation error:
    // record InvalidRecord(String data) extends Object {}

    public static void main(String[] args) {
        System.out.println("=== Inheritance Tests ===");

        testRecordImplementsInterface();
        testRecordComparable();
        testRecordInheritance();
        testRecordSuperclass();
    }

    public static void testRecordImplementsInterface() {
        System.out.println("--- Test Record Implements Interface ---");
        Circle circle = new Circle(5.0);
        circle.draw();
        System.out.println("Circle is Drawable: " + (circle instanceof Drawable));
    }

    public static void testRecordComparable() {
        System.out.println("--- Test Record Comparable ---");
        Person person1 = new Person("Alice", 30);
        Person person2 = new Person("Bob", 25);
        Person person3 = new Person("Alice", 35);

        System.out.println("person1.compareTo(person2): " + person1.compareTo(person2));
        System.out.println("person2.compareTo(person1): " + person2.compareTo(person1));
        System.out.println("person1.compareTo(person3): " + person1.compareTo(person3));
        System.out.println("person3.compareTo(person1): " + person3.compareTo(person1));
    }

    public static void testRecordInheritance() {
        System.out.println("--- Test Record Inheritance ---");
        Person person = new Person("Charlie", 40);

        // Records implicitly extend java.lang.Record
        System.out.println("Person superclass: " + person.getClass().getSuperclass().getName());
        System.out.println("Is Record instance: " + (person instanceof Record));

        // Records are final - cannot be extended
        System.out.println("Person class is final: " +
            java.lang.reflect.Modifier.isFinal(person.getClass().getModifiers()));
    }

    public static void testRecordSuperclass() {
        System.out.println("--- Test Record Superclass Methods ---");
        Circle circle = new Circle(3.0);

        // Test methods inherited from Record
        System.out.println("Circle toString: " + circle.toString());
        System.out.println("Circle equals itself: " + circle.equals(circle));
        System.out.println("Circle hashCode: " + circle.hashCode());

        Circle circle2 = new Circle(3.0);
        System.out.println("Two circles with same radius equal: " + circle.equals(circle2));
    }
}
