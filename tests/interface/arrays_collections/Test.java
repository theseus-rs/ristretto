/** Test interfaces with arrays and collections */
interface Drawable {
    void draw();
    String getName();
}

interface Colorable {
    void setColor(String color);
    String getColor();
}

class Circle implements Drawable, Colorable {
    private String color = "black";

    public void draw() {
        System.out.println("Drawing a " + color + " circle");
    }

    public String getName() {
        return "Circle";
    }

    public void setColor(String color) {
        this.color = color;
    }

    public String getColor() {
        return color;
    }
}

class Rectangle implements Drawable, Colorable {
    private String color = "black";

    public void draw() {
        System.out.println("Drawing a " + color + " rectangle");
    }

    public String getName() {
        return "Rectangle";
    }

    public void setColor(String color) {
        this.color = color;
    }

    public String getColor() {
        return color;
    }
}

class Line implements Drawable {
    public void draw() {
        System.out.println("Drawing a line");
    }

    public String getName() {
        return "Line";
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Arrays and Collections Test ===");

        // Test interface arrays
        Drawable[] drawables = new Drawable[3];
        drawables[0] = new Circle();
        drawables[1] = new Rectangle();
        drawables[2] = new Line();

        System.out.println("Drawing all shapes:");
        for (int i = 0; i < drawables.length; i++) {
            System.out.print("Index " + i + ": ");
            drawables[i].draw();
        }

        // Test enhanced for loop with interface array
        System.out.println("\nUsing enhanced for loop:");
        for (Drawable drawable : drawables) {
            System.out.println("Shape: " + drawable.getName());
            drawable.draw();
        }

        // Test array of different interface type
        Colorable[] colorables = new Colorable[2];
        colorables[0] = new Circle();
        colorables[1] = new Rectangle();

        System.out.println("\nTesting colorable shapes:");
        for (Colorable colorable : colorables) {
            colorable.setColor("red");
            System.out.println("Color set to: " + colorable.getColor());
            if (colorable instanceof Drawable) {
                ((Drawable) colorable).draw();
            }
        }

        // Test array initialization with interface references
        Drawable[] moreDrawables = {
            new Circle(),
            new Rectangle(),
            new Line()
        };

        System.out.println("\nArray initialization test:");
        for (Drawable d : moreDrawables) {
            d.draw();
        }

        // Test instanceof with arrays
        System.out.println("\nArray instanceof tests:");
        System.out.println("drawables instanceof Drawable[]: " + (drawables instanceof Drawable[]));
        System.out.println("drawables instanceof Object[]: " + (drawables instanceof Object[]));
        System.out.println("colorables instanceof Colorable[]: " + (colorables instanceof Colorable[]));
        System.out.println("colorables instanceof Object[]: " + (colorables instanceof Object[]));

        // Test array element instanceof
        System.out.println("\nArray element instanceof tests:");
        for (int i = 0; i < drawables.length; i++) {
            System.out.println("drawables[" + i + "] instanceof Drawable: " + (drawables[i] instanceof Drawable));
            System.out.println("drawables[" + i + "] instanceof Colorable: " + (drawables[i] instanceof Colorable));
            System.out.println("drawables[" + i + "] instanceof Circle: " + (drawables[i] instanceof Circle));
            System.out.println("drawables[" + i + "] instanceof Rectangle: " + (drawables[i] instanceof Rectangle));
            System.out.println("drawables[" + i + "] instanceof Line: " + (drawables[i] instanceof Line));
        }

        // Test multidimensional interface arrays
        Drawable[][] matrix = new Drawable[2][2];
        matrix[0][0] = new Circle();
        matrix[0][1] = new Rectangle();
        matrix[1][0] = new Line();
        matrix[1][1] = new Circle();

        System.out.println("\nMultidimensional array test:");
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                System.out.print("matrix[" + i + "][" + j + "]: ");
                matrix[i][j].draw();
            }
        }
    }
}
