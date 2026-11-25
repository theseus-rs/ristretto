import java.lang.classfile.ClassFile;
import java.lang.constant.*;
import java.lang.invoke.*;
import static java.lang.constant.ConstantDescs.*;

public class Test {
    static final ClassDesc OWNER = ClassDesc.of("Test");
    static final MethodTypeDesc GET = MethodTypeDesc.of(CD_Object);
    static final int FLAGS = ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC;
    static final AssertionError ERROR = new AssertionError("bootstrap");
    static final IllegalStateException EXCEPTION = new IllegalStateException("bootstrap");
    static int classArguments, targets, retryAttempts, transientAttempts;
    static Object fresh;
    static Class<?> generatedCaller;
    static class CustomLinkageError extends LinkageError {}

    public static Object receive(MethodHandles.Lookup lookup, String name, Class<?> type, Class<?> argument) {
        classArguments++;
        return argument;
    }

    public static MethodHandle handle(MethodHandles.Lookup lookup, String name, MethodType type) throws Exception {
        return lookup.findStatic(Test.class, "target", type);
    }

    public static Object target() {
        targets++;
        return "target";
    }

    public static Object site(MethodHandles.Lookup lookup, String name, MethodType type) {
        switch (name) {
            case "customError": throw new CustomLinkageError();
            case "transientError":
                if (++transientAttempts == 1) throw ERROR;
                return new ConstantCallSite(MethodHandles.constant(Object.class, "recovered"));
            case "lookup":
                return new ConstantCallSite(MethodHandles.constant(Object.class,
                        lookup.lookupClass() == generatedCaller && lookup.lookupModes() == MethodHandles.lookup().lookupModes()));
            case "retry":
                if (++retryAttempts > 1) return new ConstantCallSite(MethodHandles.constant(Object.class, "linked"));
                // The first bootstrap produces a target with the wrong return type.
            case "wrongType": return new ConstantCallSite(MethodHandles.constant(String.class, "wrong"));
            case "nullSite": return null;
            case "error": throw ERROR;
            case "exception": throw EXCEPTION;
            default: return new ConstantCallSite(MethodHandles.constant(Object.class, "ok"));
        }
    }

    public static Object noArguments() { throw new AssertionError("unexpected bootstrap call"); }

    @SuppressWarnings("removal")
    public static Object makeBox(MethodHandles.Lookup lookup, String name, Class<?> type) {
        if (type == int.class || type == Integer.class) fresh = new Integer(42);
        else if (type == char.class) fresh = new Character('a');
        else fresh = new Long(42L);
        return fresh;
    }

    public static CallSite boxIdentity(MethodHandles.Lookup lookup, String name, MethodType type, Object box) {
        boolean valid = switch (name) {
            case "integer", "integerReference" -> box == Integer.valueOf(42) && box != fresh;
            case "character" -> box == fresh && box instanceof Character c && c.charValue() == 'a';
            default -> box == fresh;
        };
        return new ConstantCallSite(MethodHandles.constant(Object.class, valid));
    }

    static Class<?> caller() throws Exception {
        var receive = ofConstantBootstrap(OWNER, "receive", CD_Object, CD_Class);
        var box = ofConstantBootstrap(OWNER, "makeBox", CD_Object);
        var inspect = ofCallsiteBootstrap(OWNER, "boxIdentity", CD_CallSite, CD_Object);
        var site = ofCallsiteBootstrap(OWNER, "site", CD_Object);
        var bytes = ClassFile.of().build(ClassDesc.of("LinkageCaller"), builder -> {
            for (var name : new String[]{"customError", "transientError", "lookup", "ok", "wrongType", "nullSite", "error", "exception", "bare", "arity"}) {
                var bootstrap = switch (name) {
                    case "bare" -> ofCallsiteBootstrap(OWNER, "handle", CD_MethodHandle);
                    case "arity" -> MethodHandleDesc.ofMethod(DirectMethodHandleDesc.Kind.STATIC, OWNER, "noArguments", GET);
                    default -> site;
                };
                var desc = DynamicCallSiteDesc.of(bootstrap, name, GET);
                builder.withMethodBody(name, GET, FLAGS, code -> code.invokedynamic(desc).areturn());
            }
            var retry = DynamicCallSiteDesc.of(site, "retry", GET);
            builder.withMethodBody("retry", GET, FLAGS, code -> code.invokedynamic(retry).areturn());
            builder.withMethodBody("otherInstruction", GET, FLAGS, code -> code.invokedynamic(retry).areturn());
            for (var name : new String[]{"integer", "character", "reference", "integerReference"}) {
                var ty = switch (name) {
                    case "integer" -> CD_int;
                    case "character" -> CD_char;
                    case "integerReference" -> CD_Integer;
                    default -> CD_Long;
                };
                var arg = DynamicConstantDesc.ofNamed(box, name, ty);
                var desc = DynamicCallSiteDesc.of(inspect, name, GET, arg);
                builder.withMethodBody(name, GET, FLAGS, code -> code.invokedynamic(desc).areturn());
            }
            var hidden = ClassDesc.of("foreign.HiddenArgument");
            for (var name : new String[]{"hidden", "hiddenArray", "internal", "visible"}) {
                var arg = switch (name) {
                    case "visible" -> ClassDesc.of("foreign.Visible$PublicArgument");
                    case "hiddenArray" -> hidden.arrayType(2);
                    case "internal" -> ClassDesc.of("jdk.internal.misc.Unsafe");
                    default -> hidden;
                };
                var desc = DynamicConstantDesc.ofNamed(receive, name, CD_Class, arg);
                builder.withMethodBody(name, GET, FLAGS, code -> code.ldc(desc).areturn());
            }
            var missing = DynamicConstantDesc.ofNamed(ofConstantBootstrap(OWNER, "absent", CD_Object),
                    "missing", ClassDesc.of("MissingDescriptor"));
            builder.withMethodBody("missing", GET, FLAGS, code -> code.ldc(missing).areturn());
            var absentMethod = ofConstantBootstrap(OWNER, "absent", CD_Object, ClassDesc.of("MissingParameter"));
            var missingMember = DynamicConstantDesc.ofNamed(absentMethod, "missingMember", CD_Object);
            builder.withMethodBody("missingMember", GET, FLAGS, code -> code.ldc(missingMember).areturn());
        });
        return MethodHandles.lookup().defineClass(bytes);
    }

    static Object run(Class<?> caller, String name) throws Throwable {
        return (Object) MethodHandles.lookup().findStatic(caller, name, MethodType.methodType(Object.class)).invokeExact();
    }

    static void expect(Class<?> caller, String name, Class<? extends Throwable> type, Class<?> causeType) throws Throwable {
        try { run(caller, name); }
        catch (Throwable t) {
            if (t.getClass() != type || (causeType != null && !causeType.isInstance(t.getCause()))
                    || (name.equals("retry") && causeType == null && t.getCause() != null)
                    || (name.equals("error") && t != ERROR) || (name.equals("exception") && t.getCause() != EXCEPTION)) {
                throw new AssertionError(name, t);
            }
            return;
        }
        throw new AssertionError("unexpected success for " + name);
    }

    public static void main(String[] args) throws Throwable {
        var caller = generatedCaller = caller();
        if (!Boolean.TRUE.equals(run(caller, "lookup"))) throw new AssertionError("bootstrap lookup modes");
        if (!"ok".equals(run(caller, "ok"))) throw new AssertionError("valid bootstrap");
        expect(caller, "bare", BootstrapMethodError.class, ClassCastException.class);
        expect(caller, "nullSite", BootstrapMethodError.class, ClassCastException.class);
        expect(caller, "wrongType", BootstrapMethodError.class, WrongMethodTypeException.class);
        expect(caller, "retry", BootstrapMethodError.class, WrongMethodTypeException.class);
        expect(caller, "retry", BootstrapMethodError.class, null);
        if (retryAttempts != 1 || !"linked".equals(run(caller, "otherInstruction")) || retryAttempts != 2)
            throw new AssertionError("per-instruction linkage failure");
        expect(caller, "customError", CustomLinkageError.class, null);
        expect(caller, "customError", NoClassDefFoundError.class, null);
        expect(caller, "transientError", AssertionError.class, null);
        if (!"recovered".equals(run(caller, "transientError")) || transientAttempts != 2)
            throw new AssertionError("retry after a non-linkage Error");
        expect(caller, "arity", BootstrapMethodError.class, WrongMethodTypeException.class);
        expect(caller, "exception", BootstrapMethodError.class, IllegalStateException.class);
        expect(caller, "error", AssertionError.class, null);
        expect(caller, "missing", NoSuchMethodError.class, null);
        expect(caller, "missingMember", NoSuchMethodError.class, null);
        expect(caller, "hidden", IllegalAccessError.class, null);
        expect(caller, "hiddenArray", IllegalAccessError.class, null);
        expect(caller, "internal", IllegalAccessError.class, null);
        if (run(caller, "visible") != foreign.Visible.PublicArgument.class || classArguments != 1
                || foreign.Visible.initializations != 0 || targets != 0) throw new AssertionError("bootstrap side effects");
        for (var name : new String[]{"integer", "character", "reference", "integerReference"}) {
            if (!Boolean.TRUE.equals(run(caller, name))) throw new AssertionError(name);
        }
        System.out.println("Bootstrap linkage passed");
    }
}
