/** Test instanceof behavior with mixed inheritance and interface scenarios */
interface Drawable {
    void draw();
}

interface Resizable {
    void resize();
}

interface Colorable {
    void setColor(String color);
}

abstract class Shape implements Drawable {
    public abstract void draw();
}

class Rectangle extends Shape implements Resizable, Colorable {
    public void draw() {}
    public void resize() {}
    public void setColor(String color) {}
}

class Circle extends Shape implements Resizable {
    public void draw() {}
    public void resize() {}
}

class Triangle extends Shape {
    public void draw() {}
}

class ColoredRectangle extends Rectangle {
    public void specialMethod() {}
}

public class Test {
    public static void main(String[] args) {
        // Test complex inheritance with interfaces
        Rectangle rect = new Rectangle();
        System.out.println("Rectangle instanceof Rectangle: " + (rect instanceof Rectangle));
        System.out.println("Rectangle instanceof Shape: " + (rect instanceof Shape));
        System.out.println("Rectangle instanceof Drawable: " + (rect instanceof Drawable));
        System.out.println("Rectangle instanceof Resizable: " + (rect instanceof Resizable));
        System.out.println("Rectangle instanceof Colorable: " + (rect instanceof Colorable));
        System.out.println("Rectangle instanceof Object: " + (rect instanceof Object));

        // Test Circle (implements fewer interfaces)
        Circle circle = new Circle();
        System.out.println("Circle instanceof Circle: " + (circle instanceof Circle));
        System.out.println("Circle instanceof Shape: " + (circle instanceof Shape));
        System.out.println("Circle instanceof Drawable: " + (circle instanceof Drawable));
        System.out.println("Circle instanceof Resizable: " + (circle instanceof Resizable));
        System.out.println("Circle instanceof Colorable: " + (circle instanceof Colorable));

        // Test Triangle (implements no additional interfaces)
        Triangle triangle = new Triangle();
        System.out.println("Triangle instanceof Triangle: " + (triangle instanceof Triangle));
        System.out.println("Triangle instanceof Shape: " + (triangle instanceof Shape));
        System.out.println("Triangle instanceof Drawable: " + (triangle instanceof Drawable));
        System.out.println("Triangle instanceof Resizable: " + (triangle instanceof Resizable));
        System.out.println("Triangle instanceof Colorable: " + (triangle instanceof Colorable));

        // Test ColoredRectangle (extended hierarchy)
        ColoredRectangle coloredRect = new ColoredRectangle();
        System.out.println("ColoredRectangle instanceof ColoredRectangle: " + (coloredRect instanceof ColoredRectangle));
        System.out.println("ColoredRectangle instanceof Rectangle: " + (coloredRect instanceof Rectangle));
        System.out.println("ColoredRectangle instanceof Shape: " + (coloredRect instanceof Shape));
        System.out.println("ColoredRectangle instanceof Drawable: " + (coloredRect instanceof Drawable));
        System.out.println("ColoredRectangle instanceof Resizable: " + (coloredRect instanceof Resizable));
        System.out.println("ColoredRectangle instanceof Colorable: " + (coloredRect instanceof Colorable));

        // Test polymorphic references through interfaces
        Drawable drawableRect = new Rectangle();
        System.out.println("Drawable ref (Rectangle) instanceof Drawable: " + (drawableRect instanceof Drawable));
        System.out.println("Drawable ref (Rectangle) instanceof Rectangle: " + (drawableRect instanceof Rectangle));
        System.out.println("Drawable ref (Rectangle) instanceof Shape: " + (drawableRect instanceof Shape));
        System.out.println("Drawable ref (Rectangle) instanceof Resizable: " + (drawableRect instanceof Resizable));
        System.out.println("Drawable ref (Rectangle) instanceof Colorable: " + (drawableRect instanceof Colorable));

        Resizable resizableCircle = new Circle();
        System.out.println("Resizable ref (Circle) instanceof Resizable: " + (resizableCircle instanceof Resizable));
        System.out.println("Resizable ref (Circle) instanceof Circle: " + (resizableCircle instanceof Circle));
        System.out.println("Resizable ref (Circle) instanceof Shape: " + (resizableCircle instanceof Shape));
        System.out.println("Resizable ref (Circle) instanceof Drawable: " + (resizableCircle instanceof Drawable));
        System.out.println("Resizable ref (Circle) instanceof Colorable: " + (resizableCircle instanceof Colorable));

        // Test abstract class reference
        Shape shapeTriangle = new Triangle();
        System.out.println("Shape ref (Triangle) instanceof Shape: " + (shapeTriangle instanceof Shape));
        System.out.println("Shape ref (Triangle) instanceof Triangle: " + (shapeTriangle instanceof Triangle));
        System.out.println("Shape ref (Triangle) instanceof Drawable: " + (shapeTriangle instanceof Drawable));
        System.out.println("Shape ref (Triangle) instanceof Resizable: " + (shapeTriangle instanceof Resizable));
        System.out.println("Shape ref (Triangle) instanceof Rectangle: " + (shapeTriangle instanceof Rectangle));

        // Test arrays of complex types
        Rectangle[] rectArray = new Rectangle[3];
        System.out.println("Rectangle[] instanceof Object: " + (rectArray instanceof Object));
        System.out.println("Rectangle[] instanceof Rectangle[]: " + (rectArray instanceof Rectangle[]));
        System.out.println("Rectangle[] instanceof Shape[]: " + (rectArray instanceof Shape[]));
        System.out.println("Rectangle[] instanceof Drawable[]: " + (rectArray instanceof Drawable[]));
        System.out.println("Rectangle[] instanceof Resizable[]: " + (rectArray instanceof Resizable[]));
        System.out.println("Rectangle[] instanceof Object[]: " + (rectArray instanceof Object[]));

        Shape[] shapeArray = new Shape[3];
        System.out.println("Shape[] instanceof Shape[]: " + (shapeArray instanceof Shape[]));
        System.out.println("Shape[] instanceof Drawable[]: " + (shapeArray instanceof Drawable[]));
        System.out.println("Shape[] instanceof Object[]: " + (shapeArray instanceof Object[]));
        System.out.println("Shape[] instanceof Rectangle[]: " + (shapeArray instanceof Rectangle[]));
    }
}
