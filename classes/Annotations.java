import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Parameter;

@Deprecated(since = "1")
public class Annotations {
    @Deprecated(since = "2")
    public static final boolean STATIC_FIELD = false;

    @Deprecated(since = "3")
    public boolean instanceField;

    @Deprecated(since = "4")
    public Annotations() {
    }

    @Deprecated(since = "5")
    public static void staticMethod() {
    }

    @Deprecated(since = "6")
    public void instanceMethod() {
    }

    public void parameterMethod(@Deprecated(since = "7") int parameter) {
    }

    public static void main(String ... args) throws Throwable {
        Class<Annotations> cls = Annotations.class;
        System.out.println("class: " + cls.getAnnotation(Deprecated.class));

        Field staticField = cls.getField("STATIC_FIELD");
        System.out.println("static field: " + staticField.getAnnotation(Deprecated.class));

        Field instanceField = cls.getField("instanceField");
        System.out.println("instance field: " + instanceField.getAnnotation(Deprecated.class));

        Constructor<Annotations> constructor = cls.getConstructor();
        System.out.println("constructor: " + constructor.getAnnotation(Deprecated.class));

        Method staticMethod = cls.getMethod("staticMethod");
        System.out.println("static method: " + staticMethod.getAnnotation(Deprecated.class));

        Method instanceMethod = cls.getMethod("instanceMethod");
        System.out.println("instance method: " + instanceMethod.getAnnotation(Deprecated.class));

        Method parameterMethod = cls.getMethod("parameterMethod", int.class);
        Parameter parameter = parameterMethod.getParameters()[0];
        System.out.println("parameter: " + parameter.getAnnotation(Deprecated.class));
    }
}

