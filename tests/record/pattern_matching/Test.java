public class Test {

    // Records for pattern matching tests
    record Point(int x, int y) {}

    record Circle(Point center, double radius) {}

    record Rectangle(Point topLeft, Point bottomRight) {}

    // Sealed interface for pattern matching
    sealed interface Shape permits CircleShape, RectangleShape {}

    record CircleShape(Point center, double radius) implements Shape {}

    record RectangleShape(Point topLeft, Point bottomRight) implements Shape {}

    public static void main(String[] args) {
        System.out.println("=== Pattern Matching Tests ===");

        testInstanceofPatternMatching();
        testSwitchExpressionPatternMatching();
        testNestedPatternMatching();
        testGuardedPatterns();
        testSealedClassPatternMatching();
    }

    public static void testInstanceofPatternMatching() {
        System.out.println("--- Test instanceof Pattern Matching ---");
        Object obj1 = new Point(10, 20);
        Object obj2 = new Circle(new Point(5, 5), 3.0);
        Object obj3 = "Not a record";

        // Pattern matching instanceof (Java 14+)
        if (obj1 instanceof Point p) {
            System.out.println("Pattern matching: Point(" + p.x() + ", " + p.y() + ")");
        }

        if (obj2 instanceof Circle c) {
            System.out.println("Circle center: " + c.center() + ", radius: " + c.radius());
        }

        if (obj3 instanceof Point) {
            System.out.println("This won't print");
        } else {
            System.out.println("obj3 is not a Point: " + obj3);
        }
    }

    public static void testSwitchExpressionPatternMatching() {
        System.out.println("--- Test Switch Expression Pattern Matching ---");
        Object[] objects = {
                new Point(1, 2),
                new Circle(new Point(0, 0), 5.0),
                "Hello",
                42,
                new Rectangle(new Point(0, 0), new Point(10, 10))
        };

        for (Object obj : objects) {
            String result = switch (obj) {
                case Point p -> "Point at (" + p.x() + ", " + p.y() + ")";
                case Circle c -> "Circle at " + c.center() + " with radius " + c.radius();
                case Rectangle r -> "Rectangle from " + r.topLeft() + " to " + r.bottomRight();
                case String s -> "String: " + s;
                case Integer i -> "Integer: " + i;
                default -> "Unknown type: " + obj.getClass().getSimpleName();
            };
            System.out.println("Object: " + result);
        }
    }

    public static void testNestedPatternMatching() {
        System.out.println("--- Test Nested Pattern Matching ---");
        Circle circle1 = new Circle(new Point(0, 0), 5.0);
        Circle circle2 = new Circle(new Point(3, 4), 2.5);
        Rectangle rect = new Rectangle(new Point(1, 1), new Point(5, 5));

        Object[] shapes = {circle1, circle2, rect};

        for (Object shape : shapes) {
            // Note: Nested pattern matching syntax may vary by Java version
            String description = switch (shape) {
                case Circle(Point(var x, var y), var radius) ->
                        "Circle at (" + x + ", " + y + ") with radius " + radius;
                case Rectangle(Point(var x1, var y1), Point(var x2, var y2)) ->
                        "Rectangle from (" + x1 + ", " + y1 + ") to (" + x2 + ", " + y2 + ")";
                default -> "Unknown shape";
            };
            System.out.println("Shape: " + description);
        }
    }

    public static void testGuardedPatterns() {
        System.out.println("--- Test Conditional Pattern Matching ---");
        Point[] points = {
                new Point(0, 0),
                new Point(5, 0),
                new Point(0, 5),
                new Point(3, 4),
                new Point(-2, 1)
        };

        for (Point point : points) {
            String classification = switch (point) {
                case Point(int x, int y) -> {
                    if (x == 0 && y == 0) yield "Origin";
                    else if (x > 0 && y == 0) yield "Positive X-axis";
                    else if (x == 0 && y > 0) yield "Positive Y-axis";
                    else if (x > 0 && y > 0) yield "First quadrant";
                    else if (x < 0 && y > 0) yield "Second quadrant";
                    else if (x < 0 && y < 0) yield "Third quadrant";
                    else if (x > 0 && y < 0) yield "Fourth quadrant";
                    else yield "On axis";
                }
            };
            System.out.println("Point " + point + " is at: " + classification);
        }
    }

    public static void testSealedClassPatternMatching() {
        System.out.println("--- Test Sealed Class Pattern Matching ---");
        Shape[] shapes = {
                new CircleShape(new Point(2, 3), 4.0),
                new RectangleShape(new Point(0, 0), new Point(6, 8))
        };

        for (Shape shape : shapes) {
            double area = switch (shape) {
                case CircleShape(Point center, double radius) -> Math.PI * radius * radius;
                case RectangleShape(Point(int x1, int y1), Point(int x2, int y2)) ->
                        Math.abs((x2 - x1) * (y2 - y1));
            };

            String info = switch (shape) {
                case CircleShape(Point center, double radius) ->
                        "Circle at " + center + " with radius " + radius;
                case RectangleShape(Point topLeft, Point bottomRight) ->
                        "Rectangle from " + topLeft + " to " + bottomRight;
            };

            System.out.println(info + " has area: " + area);
        }
    }
}
