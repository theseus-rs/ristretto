import java.io.InputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import sun.misc.Unsafe;

/** Symbolic references and reflection must retain the defining loader, even when a parent
 * already defines the same binary name. Resolution must not initialize component types. */
public class Test {
    public static int initializations;
    public static int deferredInitializations;
    public static int exceptionInitializations;

    static class Loader extends ClassLoader {
        Class<?> define(String source, String name) throws Exception {
            byte[] data;
            try (InputStream in = Test.class.getResourceAsStream("/" + source + ".class")) {
                if (in == null) throw new AssertionError("missing fixture: " + source);
                data = in.readAllBytes();
            }
            if (!name.equals(source)) {
                byte[] from = source.getBytes("UTF-8"), to = name.getBytes("UTF-8");
                check(from.length == to.length, "replacement name length");
                for (int i = 0; i <= data.length - from.length; i++) {
                    boolean match = true;
                    for (int j = 0; j < from.length; j++) if (data[i + j] != from[j]) match = false;
                    if (match) System.arraycopy(to, 0, data, i, to.length);
                }
            }
            return defineClass(name, data, 0, data.length);
        }
    }

    static void check(boolean condition, String description) {
        if (!condition) throw new AssertionError(description);
    }

    static MethodHandle nestChecker() throws Exception {
        Field field = Unsafe.class.getDeclaredField("theUnsafe");
        field.setAccessible(true);
        Unsafe unsafe = (Unsafe) field.get(null);
        Field impl = MethodHandles.Lookup.class.getDeclaredField("IMPL_LOOKUP");
        MethodHandles.Lookup lookup = (MethodHandles.Lookup) unsafe.getObject(
                unsafe.staticFieldBase(impl), unsafe.staticFieldOffset(impl));
        return lookup.findStatic(Class.forName("jdk.internal.reflect.Reflection"), "areNestMates",
                MethodType.methodType(boolean.class, Class.class, Class.class));
    }

    static void checkLoader(MethodHandle nest) throws Throwable {
        Loader loader = new Loader();
        Class<?> child = loader.define("Fixture", "Fixture");
        Class<?> member = loader.define("Fixture$Member", "Fixture$Member");
        Class<?> deferred = loader.define("Fixture$Deferred", "Fixture$Deferred");
        loader.define("ChildException", "ChildException");
        Class<?> pair = loader.define("Fixture$Pair", "Fixture$Pair");
        int before = initializations;
        Object childValue = Array.newInstance(child, 1);
        Class<?> childArray = childValue.getClass();
        Class<?> parentArray = Fixture[].class;
        Object parentValue = new Fixture[1];
        Class<?> deferredArray = Array.newInstance(deferred, 1).getClass();
        check(childArray.getComponentType() == child && childArray.getClassLoader() == loader, "array mirrors");
        check(!parentArray.isAssignableFrom(childArray), "array assignability isolated");
        check(!childArray.isAssignableFrom(parentArray), "reverse array assignability isolated");
        check(childArray.isAssignableFrom(deferredArray), "array covariance within loader");
        check(!parentArray.isAssignableFrom(deferredArray), "array covariance across loaders");
        check(!parentArray.isInstance(childValue) && childArray.isInstance(childValue), "array isInstance");
        check(!(childValue instanceof Fixture[]), "array instanceof");
        try { Fixture[] ignored = (Fixture[]) childValue; throw new AssertionError("array checkcast accepted"); }
        catch (ClassCastException expected) {}
        Class<?> childMatrix = Array.newInstance(child, 1, 1).getClass();
        check(!Fixture[][].class.isAssignableFrom(childMatrix), "multidimensional assignability");
        check(initializations == before && deferredInitializations == 0, "array checks initialized components");

        check(child.isNestmateOf(member) && !Fixture.class.isNestmateOf(member), "public nest identity");
        check((boolean) nest.invokeExact(child, member), "internal own nest");
        check(!(boolean) nest.invokeExact(Fixture.class, member), "internal cross-loader nest");
        check(!(boolean) nest.invokeExact(Fixture.class, child), "same-named non-nestmates");
        check(initializations == before, "nest checks initialized classes");

        check(child.getMethod("single").invoke(null).getClass() == childArray, "anewarray loader");
        Object matrix = child.getMethod("multi").invoke(null);
        check(matrix.getClass() == childMatrix && Array.get(matrix, 0).getClass() == childArray, "multianewarray loader");
        Object partial = child.getMethod("partial").invoke(null);
        check(partial.getClass() == Array.newInstance(child, 0, 0, 0).getClass(), "partial array rank");
        check(Array.get(Array.get(partial, 0), 0) == null, "partial array leaves");
        check(child.getMethod("deferredArray").invoke(null).getClass() == deferredArray, "deferred component loader");
        Class<?> literal = (Class<?>) child.getMethod("arrayLiteral").invoke(null);
        check(literal.getComponentType() == deferredArray, "ldc array component mirror");
        check(Array.newInstance(literal, 1).getClass().getComponentType() == literal, "ldc mirror registry");
        check(deferredInitializations == 0, "anewarray initialized component");
        Method isArray = child.getMethod("isArray", Object.class);
        check(Boolean.TRUE.equals(isArray.invoke(null, childValue)), "custom instanceof positive");
        check(Boolean.FALSE.equals(isArray.invoke(null, parentValue)), "custom instanceof isolated");
        MethodHandle castArray = MethodHandles.lookup().findStatic(child, "castArray",
                MethodType.methodType(Object.class, Object.class));
        check((Object) castArray.invokeExact(childValue) == childValue, "custom checkcast positive");
        try { Object ignored = (Object) castArray.invokeExact(parentValue); throw new AssertionError("custom checkcast accepted"); }
        catch (ClassCastException expected) {}
        int exceptionsBefore = exceptionInitializations;
        check("caught runtime".equals(child.getMethod("unmatchedCatch").invoke(null)), "unmatched custom catch");
        check(exceptionInitializations == exceptionsBefore, "catch type resolution initialized exception");
        check("caught".equals(child.getMethod("caught").invoke(null)), "custom exception catch");
        child.getMethod("setLabel", Object.class).invoke(null, "child");
        check("child".equals(child.getMethod("supplier").invoke(null)), "custom method reference");
        check("Pair[value=null]".equals(pair.getConstructor(Object.class).newInstance(new Object[] {null}).toString()), "custom record");
        System.out.println("loader references preserve identity without premature initialization");
    }

    public static void main(String[] args) throws Throwable {
        // Resolve parent definitions before running their child counterparts.
        Class<?>[] parents = {Fixture.class, Fixture.Member.class, Fixture.Deferred.class,
                Fixture.Pair.class, ChildException.class};
        MethodHandle nest = nestChecker();
        checkLoader(nest);
        checkLoader(nest);
        Class<?> missing = new Loader().define("Fixture", "Missing");
        int before = initializations;
        Method method = missing.getMethod("param", Object.class);
        check(method.getParameters()[0].getName().equals("value"), "custom-only parameter metadata");
        check(method.getAnnotatedReturnType().getType() == Object.class, "custom-only method annotations");
        check(missing.getField("value").getAnnotatedType().getType() == Object.class, "custom-only field annotations");
        check(initializations == before, "metadata queries initialized declaring class");
        System.out.println("custom-only reflection metadata resolves without initialization");
    }
}
