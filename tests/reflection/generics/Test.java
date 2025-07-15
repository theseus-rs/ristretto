/** Test generic type reflection operations. */
import java.lang.reflect.*;
import java.util.*;

public class Test {
    static class GenericClass<T, U extends Number> {
        public T genericField;
        public List<String> stringList;
        public Map<String, Integer> stringIntMap;
        public List<? extends Number> wildcardList;
        public List<? super Integer> superWildcardList;
        public T[] genericArray;

        public <V> V genericMethod(T param1, V param2, List<? extends T> param3) {
            return param2;
        }

        public List<String> getStringList() { return stringList; }
        public void setGenericField(T value) { this.genericField = value; }
    }

    static class ConcreteClass extends GenericClass<String, Integer> {
        public void concreteMethod() {}
    }

    public static void main(String[] args) throws Exception {
        // Test generic class type parameters
        Class<?> genericClass = GenericClass.class;
        TypeVariable<?>[] typeParameters = genericClass.getTypeParameters();
        System.out.println("Generic class type parameters count: " + typeParameters.length);

        for (TypeVariable<?> typeParam : typeParameters) {
            System.out.println("Type parameter: " + typeParam.getName());
            Type[] bounds = typeParam.getBounds();
            System.out.println("Bounds count: " + bounds.length);
            for (Type bound : bounds) {
                System.out.println("Bound: " + bound.getTypeName());
            }
        }

        // Test generic superclass
        Class<?> concreteClass = ConcreteClass.class;
        Type genericSuperclass = concreteClass.getGenericSuperclass();
        System.out.println("Generic superclass: " + genericSuperclass.getTypeName());

        if (genericSuperclass instanceof ParameterizedType) {
            ParameterizedType paramType = (ParameterizedType) genericSuperclass;
            Type[] actualTypeArgs = paramType.getActualTypeArguments();
            System.out.println("Actual type arguments count: " + actualTypeArgs.length);
            for (Type arg : actualTypeArgs) {
                System.out.println("Type argument: " + arg.getTypeName());
            }
        }

        // Test generic field types
        Field stringListField = genericClass.getField("stringList");
        Type stringListType = stringListField.getGenericType();
        System.out.println("String list field type: " + stringListType.getTypeName());

        if (stringListType instanceof ParameterizedType) {
            ParameterizedType paramType = (ParameterizedType) stringListType;
            Type[] typeArgs = paramType.getActualTypeArguments();
            System.out.println("List type argument: " + typeArgs[0].getTypeName());
        }

        // Test wildcard types
        Field wildcardField = genericClass.getField("wildcardList");
        Type wildcardType = wildcardField.getGenericType();
        System.out.println("Wildcard field type: " + wildcardType.getTypeName());

        if (wildcardType instanceof ParameterizedType) {
            ParameterizedType paramType = (ParameterizedType) wildcardType;
            Type[] typeArgs = paramType.getActualTypeArguments();
            if (typeArgs[0] instanceof WildcardType) {
                WildcardType wildcard = (WildcardType) typeArgs[0];
                Type[] upperBounds = wildcard.getUpperBounds();
                Type[] lowerBounds = wildcard.getLowerBounds();
                System.out.println("Upper bounds count: " + upperBounds.length);
                System.out.println("Lower bounds count: " + lowerBounds.length);
                if (upperBounds.length > 0) {
                    System.out.println("Upper bound: " + upperBounds[0].getTypeName());
                }
            }
        }

        // Test generic method
        Method genericMethod = genericClass.getMethod("genericMethod", Object.class, Object.class, List.class);
        TypeVariable<?>[] methodTypeParams = genericMethod.getTypeParameters();
        System.out.println("Generic method type parameters count: " + methodTypeParams.length);

        Type[] paramTypes = genericMethod.getGenericParameterTypes();
        System.out.println("Generic parameter types count: " + paramTypes.length);
        for (int i = 0; i < paramTypes.length; i++) {
            System.out.println("Parameter " + i + " type: " + paramTypes[i].getTypeName());
        }

        Type returnType = genericMethod.getGenericReturnType();
        System.out.println("Generic return type: " + returnType.getTypeName());

        // Test generic array
        Field genericArrayField = genericClass.getField("genericArray");
        Type genericArrayType = genericArrayField.getGenericType();
        System.out.println("Generic array type: " + genericArrayType.getTypeName());

        if (genericArrayType instanceof GenericArrayType) {
            GenericArrayType arrayType = (GenericArrayType) genericArrayType;
            Type componentType = arrayType.getGenericComponentType();
            System.out.println("Generic array component type: " + componentType.getTypeName());
        }

        // Test map with multiple type parameters
        Field mapField = genericClass.getField("stringIntMap");
        Type mapType = mapField.getGenericType();
        System.out.println("Map field type: " + mapType.getTypeName());

        if (mapType instanceof ParameterizedType) {
            ParameterizedType paramType = (ParameterizedType) mapType;
            Type[] typeArgs = paramType.getActualTypeArguments();
            System.out.println("Map key type: " + typeArgs[0].getTypeName());
            System.out.println("Map value type: " + typeArgs[1].getTypeName());
        }

        // Test raw type
        Type rawType = ((ParameterizedType) stringListType).getRawType();
        System.out.println("Raw type: " + rawType.getTypeName());

        // Test owner type for nested generic classes
        System.out.println("Owner type: " + ((ParameterizedType) stringListType).getOwnerType());
    }
}
