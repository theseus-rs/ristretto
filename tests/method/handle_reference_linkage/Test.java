import java.lang.classfile.ClassFile;
import java.lang.constant.*;
import java.lang.invoke.*;
import static java.lang.constant.ConstantDescs.*;

public class Test {
    interface InterfaceOwner { static void method() {} }
    public static void method() {}

    static final int FLAGS = ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC;
    static final MethodTypeDesc VOID = MethodTypeDesc.of(CD_void);
    static final MethodTypeDesc HANDLE = MethodTypeDesc.of(CD_MethodHandle);

    public static void main(String[] args) throws Throwable {
        var lookup = MethodHandles.lookup();
        var owner = ClassDesc.of("PrivateOwner");
        var missing = ClassDesc.of("UnavailableParameter");
        var signature = MethodTypeDesc.of(CD_void, missing);
        lookup.defineClass(ClassFile.of().build(owner, b -> {
            b.withField("field", missing, ClassFile.ACC_PRIVATE | ClassFile.ACC_STATIC);
            b.withMethodBody("secret", signature, ClassFile.ACC_PRIVATE | ClassFile.ACC_STATIC, code -> code.return_());
        }));
        var caller = lookup.defineClass(ClassFile.of().build(ClassDesc.of("HandleCaller"), b -> {
            b.withMethodBody("methodRef", HANDLE, FLAGS, code -> code.ldc(MethodHandleDesc.ofMethod(
                    DirectMethodHandleDesc.Kind.STATIC, ClassDesc.of("Test$InterfaceOwner"), "method", VOID)).areturn());
            b.withMethodBody("interfaceRef", HANDLE, FLAGS, code -> code.ldc(MethodHandleDesc.ofMethod(
                    DirectMethodHandleDesc.Kind.INTERFACE_STATIC, ClassDesc.of("Test"), "method", VOID)).areturn());
            b.withMethodBody("inaccessible", HANDLE, FLAGS, code -> code.ldc(MethodHandleDesc.ofMethod(
                    DirectMethodHandleDesc.Kind.STATIC, owner, "secret", signature)).areturn());
            b.withMethodBody("inaccessibleField", HANDLE, FLAGS, code -> code.ldc(MethodHandleDesc.ofField(
                    DirectMethodHandleDesc.Kind.STATIC_GETTER, owner, "field", missing)).areturn());
        }));
        for (String name : new String[]{"methodRef", "interfaceRef", "inaccessible", "inaccessibleField"}) {
            MethodHandle load = lookup.findStatic(caller, name, MethodType.methodType(MethodHandle.class));
            try {
                MethodHandle value = (MethodHandle) load.invokeExact();
                throw new AssertionError(name + " resolved to " + value);
            } catch (LinkageError error) {
                var expected = switch (name) {
                    case "inaccessible" -> IllegalAccessError.class;
                    case "inaccessibleField" -> NoClassDefFoundError.class;
                    default -> IncompatibleClassChangeError.class;
                };
                if (error.getClass() != expected) throw new AssertionError(name, error);
            }
        }
        System.out.println("Handle reference linkage passed");
    }
}
