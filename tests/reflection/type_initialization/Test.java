public class Test {
    static int initialized;
    static class FieldType { static { initialized++; } }
    static class ParameterType { static { initialized++; } }
    static class ReturnType { static { initialized++; } }
    static class CheckedType extends Exception { static { initialized++; } }
    static class Inspected {
        static class Nested { static { initialized++; } }
        static { initialized++; }
        FieldType field;
        Inspected(ParameterType parameter) throws CheckedType { }
        ReturnType method(ParameterType parameter) throws CheckedType { return null; }
    }
    public static void main(String[] args) throws Exception {
        System.out.println("Interfaces: " + Inspected.class.getInterfaces().length);
        System.out.println(Inspected.class.getDeclaredFields()[0].getType().getSimpleName());
        System.out.println(Inspected.class.getDeclaredConstructors()[0].getParameterTypes()[0].getSimpleName());
        System.out.println(Inspected.class.getDeclaredMethods()[0].getReturnType().getSimpleName());
        System.out.println(Inspected.class.getDeclaredMethods()[0].getExceptionTypes()[0].getSimpleName());
        System.out.println("Nested: " + Inspected.class.getDeclaredClasses()[0].getSimpleName());
        System.out.println("Declaring: " + Inspected.Nested.class.getDeclaringClass().getSimpleName());
        System.out.println("Component: " + FieldType[].class.getComponentType().getSimpleName());
        System.out.println("Nest members: " + Test.class.getNestMembers().length);
        System.out.println("Initialized: " + initialized);
    }
}
