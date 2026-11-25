import java.lang.classfile.ClassFile;
import java.lang.constant.*;
import java.lang.invoke.*;
import static java.lang.constant.ConstantDescs.*;

public class Test {
    static final String[] STRINGS = {"a", "b", "c"};
    static int attempts, failures, earlyAttempts;

    public static Object strings(MethodHandles.Lookup lookup, String name, Class<?> type) {
        return STRINGS;
    }

    public static Object identity(MethodHandles.Lookup lookup, String name, Class<?> type, String literal) {
        return name == "hello" && literal == "hello" ? 1 : 0;
    }

    public static Object retry(MethodHandles.Lookup lookup, String name, Class<?> type) {
        if (++attempts == 1) throw new AssertionError();
        return 42;
    }

    public static Object fail(MethodHandles.Lookup lookup, String name, Class<?> type) {
        failures++;
        throw new LinkageError();
    }

    public static Object count(MethodHandles.Lookup lookup, String name, Class<?> type) {
        return ++earlyAttempts;
    }

    public static CallSite primitive(MethodHandles.Lookup lookup, String name, MethodType type, int value) {
        return new ConstantCallSite(MethodHandles.constant(int.class, value));
    }

    public static CallSite boxed(MethodHandles.Lookup lookup, String name, MethodType type, Object value) {
        return new ConstantCallSite(MethodHandles.constant(int.class, ((Integer) value).intValue()));
    }

    public static CallSite fixed(MethodHandles.Lookup lookup, String name, MethodType type, String[] values) {
        return new ConstantCallSite(MethodHandles.constant(int.class, values == STRINGS ? values.length : -1));
    }

    public static CallSite variable(MethodHandles.Lookup lookup, String name, MethodType type, String... values) {
        return new ConstantCallSite(MethodHandles.constant(int.class, values.length));
    }

    static final ClassDesc OWNER = ClassDesc.of("Test");
    static final MethodTypeDesc INT = MethodTypeDesc.of(CD_int);
    static final int FLAGS = ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC;

    static DynamicConstantDesc<?> constant(String method, String name, ClassDesc type) {
        return DynamicConstantDesc.ofNamed(ofConstantBootstrap(OWNER, method, CD_Object), name, type);
    }

    public static void main(String[] args) throws Throwable {
        var number = DynamicConstantDesc.ofNamed(BSM_EXPLICIT_CAST, "number", CD_int, 42);
        var array = constant("strings", "array", CD_String.arrayType());
        var identity = DynamicConstantDesc.ofNamed(ofConstantBootstrap(OWNER, "identity", CD_Object, CD_String),
                "hello", CD_int, "hello");
        var retry = constant("retry", "retry", CD_int);
        var fail = constant("fail", "fail", CD_int);
        // A missing bootstrap fails before evaluating the static arguments.
        var missing = DynamicConstantDesc.ofNamed(ofConstantBootstrap(OWNER, "absent", CD_Object, CD_int),
                "missing", CD_int, constant("count", "count", CD_int));
        var lateClass = ClassDesc.of("DefinedAfterResolution");
        var unresolved = DynamicConstantDesc.ofNamed(BSM_EXPLICIT_CAST, "unresolved", CD_Class, lateClass);
        var bytes = ClassFile.of().build(ClassDesc.of("BootstrapCaller"), builder -> {
            var getter = MethodHandleDesc.ofField(DirectMethodHandleDesc.Kind.STATIC_GETTER, OWNER, "attempts", CD_int);
            builder.withMethodBody("getter", MethodTypeDesc.of(CD_MethodHandle), FLAGS, code -> code.ldc(getter).areturn());
            for (var name : new String[]{"firstGetter", "secondGetter"}) {
                var entry = DynamicConstantDesc.ofNamed(BSM_EXPLICIT_CAST, name, CD_MethodHandle, getter);
                builder.withMethodBody(name, MethodTypeDesc.of(CD_MethodHandle), FLAGS, code -> code.ldc(entry).areturn());
            }
            for (var entry : new DynamicConstantDesc<?>[]{identity, retry, fail, missing}) {
                builder.withMethodBody(entry.constantName(), INT, FLAGS, code -> code.ldc(entry).ireturn());
            }
            builder.withMethodBody("unresolved", MethodTypeDesc.of(CD_Class), FLAGS,
                    code -> code.ldc(unresolved).areturn());
            var sites = new DynamicCallSiteDesc[]{
                DynamicCallSiteDesc.of(ofCallsiteBootstrap(OWNER, "primitive", CD_CallSite, CD_int), "primitive", INT, number),
                DynamicCallSiteDesc.of(ofCallsiteBootstrap(OWNER, "boxed", CD_CallSite, CD_Object), "boxed", INT, number),
                DynamicCallSiteDesc.of(ofCallsiteBootstrap(OWNER, "fixed", CD_CallSite, CD_String.arrayType()), "fixed", INT, array),
                DynamicCallSiteDesc.of(ofCallsiteBootstrap(OWNER, "variable", CD_CallSite, CD_String.arrayType()), "variable", INT, "x", "y")
            };
            for (var site : sites) {
                builder.withMethodBody(site.invocationName(), INT, FLAGS, code -> code.invokedynamic(site).ireturn());
            }
        });
        var caller = MethodHandles.lookup().defineClass(bytes);
        var lookup = MethodHandles.lookup();
        var getterType = MethodType.methodType(MethodHandle.class);
        MethodHandle first = (MethodHandle) lookup.findStatic(caller, "firstGetter", getterType).invokeExact();
        MethodHandle second = (MethodHandle) lookup.findStatic(caller, "secondGetter", getterType).invokeExact();
        check(first == second, "shared method handle constant identity");
        check(first == (MethodHandle) lookup.findStatic(caller, "getter", getterType).invokeExact(),
                "ldc and bootstrap argument identity");
        check(load(caller, "primitive") == 42, "primitive bootstrap parameter");
        check(load(caller, "boxed") == 42, "reference bootstrap parameter");
        check(load(caller, "fixed") == 3, "fixed array identity");
        check(load(caller, "variable") == 2, "variable arity");
        check(load(caller, "hello") == 1, "interned bootstrap strings");
        try {
            load(caller, "retry");
            throw new IllegalStateException("expected AssertionError");
        } catch (AssertionError expected) {}
        check(load(caller, "retry") == 42 && attempts == 2, "retry after AssertionError");
        for (int i = 0; i < 2; i++) {
            try {
                load(caller, "fail");
                throw new AssertionError("expected LinkageError");
            } catch (LinkageError expected) {
                check(expected.getClass() == LinkageError.class, "linkage error class");
            }
            try {
                load(caller, "missing");
                throw new AssertionError("expected NoSuchMethodError");
            } catch (NoSuchMethodError expected) {}
        }
        check(failures == 1 && earlyAttempts == 0, "bootstrap invocation counts");
        MethodHandle resolve = MethodHandles.lookup().findStatic(caller, "unresolved", MethodType.methodType(Class.class));
        for (int i = 0; i < 2; i++) {
            try {
                Class<?> value = (Class<?>) resolve.invokeExact();
                throw new AssertionError("unexpectedly linked " + value);
            } catch (NoClassDefFoundError expected) {
                if (i == 0) check(expected.getCause() instanceof ClassNotFoundException, "missing class cause");
            }
            if (i == 0) MethodHandles.lookup().defineClass(ClassFile.of().build(lateClass, builder -> {}));
        }
        System.out.println("Dynamic bootstrap arguments passed");
    }

    static int load(Class<?> owner, String name) throws Throwable {
        MethodHandle target = MethodHandles.lookup().findStatic(owner, name, MethodType.methodType(int.class));
        return (int) target.invokeExact();
    }

    static void check(boolean condition, String description) {
        if (!condition) throw new AssertionError(description);
    }
}
