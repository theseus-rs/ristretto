import java.lang.annotation.*;

// Inheritable annotation
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@Inherited
@interface InheritableAnnotation {
    String value() default "inherited";
    int level() default 1;
}

// Non-inheritable annotation
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@interface NonInheritableAnnotation {
    String value() default "not inherited";
}

// Method-level annotation (inheritance doesn't apply to methods)
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@Inherited
@interface MethodAnnotation {
    String value() default "method";
}

// Interface annotation
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@Inherited
@interface InterfaceAnnotation {
    String value() default "interface";
}

@InterfaceAnnotation("TestInterface")
interface TestInterface {
    @MethodAnnotation("interfaceMethod")
    void interfaceMethod();
}

@InheritableAnnotation(value = "BaseClass", level = 1)
@NonInheritableAnnotation("base")
class BaseClass implements TestInterface {

    @MethodAnnotation("baseMethod")
    public void baseMethod() {
    }

    @Override
    @MethodAnnotation("overriddenInterfaceMethod")
    public void interfaceMethod() {
    }
}

@InheritableAnnotation(value = "MiddleClass", level = 2)
class MiddleClass extends BaseClass {

    @Override
    public void baseMethod() {
        super.baseMethod();
    }

    @MethodAnnotation("middleMethod")
    public void middleMethod() {
    }
}

// Child class with no annotations - should inherit from parent
class ChildClass extends MiddleClass {

    @Override
    public void baseMethod() {
        super.baseMethod();
    }

    @Override
    public void middleMethod() {
        super.middleMethod();
    }
}

// Grandchild with new annotation - should have both inherited and own
@InheritableAnnotation(value = "GrandchildClass", level = 4)
class GrandchildClass extends ChildClass {
}

public class Test {

    public static void main(String[] args) {
        System.out.println("=== Annotation Inheritance Test ===");

        // Test inheritance hierarchy
        testClassInheritance();

        // Test interface annotation inheritance
        testInterfaceInheritance();

        // Test method annotation inheritance (should not inherit)
        testMethodInheritance();

        // Test annotation overriding
        testAnnotationOverriding();

        // Test getDeclaredAnnotations vs getAnnotations for inheritance
        testInheritanceReflection();
    }

    private static void testClassInheritance() {
        System.out.println("=== Class Inheritance Test ===");

        Class<?>[] classes = {BaseClass.class, MiddleClass.class, ChildClass.class, GrandchildClass.class};
        String[] classNames = {"BaseClass", "MiddleClass", "ChildClass", "GrandchildClass"};

        for (int i = 0; i < classes.length; i++) {
            Class<?> clazz = classes[i];
            String name = classNames[i];

            System.out.println("\n" + name + ":");

            // Test inheritable annotation
            InheritableAnnotation inherited = clazz.getAnnotation(InheritableAnnotation.class);
            System.out.println("  Has InheritableAnnotation: " + (inherited != null));
            if (inherited != null) {
                System.out.println("    value: " + inherited.value());
                System.out.println("    level: " + inherited.level());
            }

            // Test non-inheritable annotation
            NonInheritableAnnotation nonInherited = clazz.getAnnotation(NonInheritableAnnotation.class);
            System.out.println("  Has NonInheritableAnnotation: " + (nonInherited != null));
            if (nonInherited != null) {
                System.out.println("    value: " + nonInherited.value());
            }

            // Show all annotations
            Annotation[] allAnnotations = clazz.getAnnotations();
            System.out.println("  Total annotations: " + allAnnotations.length);

            // Show declared annotations (not inherited)
            Annotation[] declaredAnnotations = clazz.getDeclaredAnnotations();
            System.out.println("  Declared annotations: " + declaredAnnotations.length);
        }
    }

    private static void testInterfaceInheritance() {
        System.out.println("\n=== Interface Inheritance Test ===");

        // Note: Interface annotations are NOT inherited by implementing classes
        Class<?> interfaceClass = TestInterface.class;
        Class<?> baseClass = BaseClass.class;

        System.out.println("Interface has InterfaceAnnotation: " +
                         interfaceClass.isAnnotationPresent(InterfaceAnnotation.class));
        System.out.println("BaseClass has InterfaceAnnotation: " +
                         baseClass.isAnnotationPresent(InterfaceAnnotation.class));

        InterfaceAnnotation interfaceAnn = interfaceClass.getAnnotation(InterfaceAnnotation.class);
        if (interfaceAnn != null) {
            System.out.println("Interface annotation value: " + interfaceAnn.value());
        }

        // Check implementing classes
        Class<?>[] implementingClasses = {BaseClass.class, MiddleClass.class, ChildClass.class};
        String[] implementingNames = {"BaseClass", "MiddleClass", "ChildClass"};

        for (int i = 0; i < implementingClasses.length; i++) {
            Class<?> clazz = implementingClasses[i];
            String name = implementingNames[i];

            System.out.println(name + " has InterfaceAnnotation: " +
                             clazz.isAnnotationPresent(InterfaceAnnotation.class));
        }
    }

    private static void testMethodInheritance() {
        System.out.println("\n=== Method Inheritance Test ===");

        // Method annotations are NOT inherited even with @Inherited
        try {
            java.lang.reflect.Method baseMethod = BaseClass.class.getDeclaredMethod("baseMethod");
            java.lang.reflect.Method middleMethod = MiddleClass.class.getDeclaredMethod("baseMethod");
            java.lang.reflect.Method childMethod = ChildClass.class.getDeclaredMethod("baseMethod");

            System.out.println("BaseClass.baseMethod has MethodAnnotation: " +
                             baseMethod.isAnnotationPresent(MethodAnnotation.class));
            System.out.println("MiddleClass.baseMethod has MethodAnnotation: " +
                             middleMethod.isAnnotationPresent(MethodAnnotation.class));
            System.out.println("ChildClass.baseMethod has MethodAnnotation: " +
                             childMethod.isAnnotationPresent(MethodAnnotation.class));

            // Test interface method inheritance
            java.lang.reflect.Method interfaceMethod = TestInterface.class.getDeclaredMethod("interfaceMethod");
            java.lang.reflect.Method baseInterfaceMethod = BaseClass.class.getDeclaredMethod("interfaceMethod");

            System.out.println("Interface.interfaceMethod has MethodAnnotation: " +
                             interfaceMethod.isAnnotationPresent(MethodAnnotation.class));
            System.out.println("BaseClass.interfaceMethod has MethodAnnotation: " +
                             baseInterfaceMethod.isAnnotationPresent(MethodAnnotation.class));

        } catch (Exception e) {
            System.out.println("Error testing method inheritance: " + e.getMessage());
        }
    }

    private static void testAnnotationOverriding() {
        System.out.println("\n=== Annotation Overriding Test ===");

        // Test how annotation values change in inheritance hierarchy
        InheritableAnnotation baseAnn = BaseClass.class.getAnnotation(InheritableAnnotation.class);
        InheritableAnnotation middleAnn = MiddleClass.class.getAnnotation(InheritableAnnotation.class);
        InheritableAnnotation childAnn = ChildClass.class.getAnnotation(InheritableAnnotation.class);
        InheritableAnnotation grandchildAnn = GrandchildClass.class.getAnnotation(InheritableAnnotation.class);

        System.out.println("BaseClass annotation - value: " + (baseAnn != null ? baseAnn.value() : "null") +
                         ", level: " + (baseAnn != null ? baseAnn.level() : "null"));
        System.out.println("MiddleClass annotation - value: " + (middleAnn != null ? middleAnn.value() : "null") +
                         ", level: " + (middleAnn != null ? middleAnn.level() : "null"));
        System.out.println("ChildClass annotation - value: " + (childAnn != null ? childAnn.value() : "null") +
                         ", level: " + (childAnn != null ? childAnn.level() : "null"));
        System.out.println("GrandchildClass annotation - value: " + (grandchildAnn != null ? grandchildAnn.value() : "null") +
                         ", level: " + (grandchildAnn != null ? grandchildAnn.level() : "null"));

        // Test annotation identity
        System.out.println("MiddleClass overrides BaseClass annotation: " +
                         (middleAnn != null && baseAnn != null && !middleAnn.equals(baseAnn)));
        System.out.println("ChildClass inherits from MiddleClass: " +
                         (childAnn != null && middleAnn != null && childAnn.equals(middleAnn)));
        System.out.println("GrandchildClass overrides ChildClass annotation: " +
                         (grandchildAnn != null && childAnn != null && !grandchildAnn.equals(childAnn)));
    }

    private static void testInheritanceReflection() {
        System.out.println("\n=== Inheritance Reflection Test ===");

        Class<?> childClass = ChildClass.class;

        // getAnnotations() returns inherited annotations
        Annotation[] allAnnotations = childClass.getAnnotations();
        System.out.println("ChildClass.getAnnotations() count: " + allAnnotations.length);
        for (Annotation ann : allAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName() + ": " + ann);
        }

        // getDeclaredAnnotations() returns only directly declared annotations
        Annotation[] declaredAnnotations = childClass.getDeclaredAnnotations();
        System.out.println("ChildClass.getDeclaredAnnotations() count: " + declaredAnnotations.length);
        for (Annotation ann : declaredAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName() + ": " + ann);
        }

        // Test isAnnotationPresent for inherited vs declared
        System.out.println("ChildClass.isAnnotationPresent(InheritableAnnotation): " +
                         childClass.isAnnotationPresent(InheritableAnnotation.class));

        // Test annotation source tracking
        InheritableAnnotation inherited = childClass.getAnnotation(InheritableAnnotation.class);
        if (inherited != null) {
            System.out.println("Inherited annotation value: " + inherited.value());
            System.out.println("Inherited annotation level: " + inherited.level());

            // Check if it's the same instance as parent
            InheritableAnnotation parentAnn = MiddleClass.class.getAnnotation(InheritableAnnotation.class);
            System.out.println("Same instance as parent: " + (inherited == parentAnn));
            System.out.println("Equal to parent: " + (inherited.equals(parentAnn)));
        }
    }
}
