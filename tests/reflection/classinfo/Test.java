/** Test class metadata reflection operations. */
public class Test {
    interface InterfaceA {}
    interface InterfaceB {}

    static abstract class AbstractParent implements InterfaceA {
        abstract void abstractMethod();
    }

    static class ConcreteChild extends AbstractParent implements InterfaceB {
        void abstractMethod() {}

        static class NestedStatic {}
        class InnerClass {}
    }

    static enum TestEnum {
        VALUE1, VALUE2
    }

    @interface TestAnnotation {
        String value() default "test";
    }

    @TestAnnotation("annotated")
    static class AnnotatedClass {}

    public static void main(String[] args) throws Exception {
        // Test basic class information
        Class<?> childClass = Class.forName("Test$ConcreteChild");
        System.out.println("Class name: " + childClass.getName());
        System.out.println("Simple name: " + childClass.getSimpleName());
        System.out.println("Canonical name: " + childClass.getCanonicalName());

        // Test package information
        Package pkg = childClass.getPackage();
        System.out.println("Package: " + (pkg != null ? pkg.getName() : "null"));

        // Test superclass
        Class<?> superclass = childClass.getSuperclass();
        System.out.println("Superclass: " + superclass.getName());

        // Test interfaces
        Class<?>[] interfaces = childClass.getInterfaces();
        System.out.println("Direct interfaces count: " + interfaces.length);
        for (Class<?> iface : interfaces) {
            System.out.println("Interface: " + iface.getName());
        }

        // Test class modifiers
        System.out.println("Class modifiers: " + java.lang.reflect.Modifier.toString(childClass.getModifiers()));

        // Test class type checks
        System.out.println("Is interface: " + childClass.isInterface());
        System.out.println("Is abstract: " + java.lang.reflect.Modifier.isAbstract(childClass.getModifiers()));
        System.out.println("Is final: " + java.lang.reflect.Modifier.isFinal(childClass.getModifiers()));

        // Test enum class
        Class<?> enumClass = Class.forName("Test$TestEnum");
        System.out.println("Is enum: " + enumClass.isEnum());
        Object[] enumConstants = enumClass.getEnumConstants();
        System.out.println("Enum constants count: " + enumConstants.length);

        // Test annotation class
        Class<?> annotationClass = Class.forName("Test$TestAnnotation");
        System.out.println("Is annotation: " + annotationClass.isAnnotation());

        // Test annotated class
        Class<?> annotatedClass = Class.forName("Test$AnnotatedClass");
        java.lang.annotation.Annotation[] annotations = annotatedClass.getAnnotations();
        System.out.println("Annotations count: " + annotations.length);

        // Test nested classes
        Class<?>[] nestedClasses = Test.class.getDeclaredClasses();
        System.out.println("Nested classes count: " + nestedClasses.length);

        // Test enclosing class
        Class<?> nestedClass = Class.forName("Test$ConcreteChild$NestedStatic");
        Class<?> enclosingClass = nestedClass.getEnclosingClass();
        System.out.println("Enclosing class: " + enclosingClass.getName());

        // Test array class
        Class<?> arrayClass = String[].class;
        System.out.println("Is array: " + arrayClass.isArray());
        System.out.println("Component type: " + arrayClass.getComponentType().getName());

        // Test primitive class
        Class<?> primitiveClass = int.class;
        System.out.println("Is primitive: " + primitiveClass.isPrimitive());

        // Test assignability
        System.out.println("Child assignable from parent: " + AbstractParent.class.isAssignableFrom(childClass));
        System.out.println("Parent assignable from child: " + childClass.isAssignableFrom(AbstractParent.class));

        // Test instance checking
        ConcreteChild instance = new ConcreteChild();
        System.out.println("Instance is of child class: " + childClass.isInstance(instance));
        System.out.println("Instance is of parent class: " + AbstractParent.class.isInstance(instance));
    }
}

