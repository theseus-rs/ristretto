import java.lang.classfile.ClassFile;
import java.lang.constant.ClassDesc;
import java.lang.constant.MethodTypeDesc;
import java.lang.invoke.*;
import static java.lang.constant.ConstantDescs.*;

public class SubwordAtomics {
    public byte b;
    public short s;
    public char c;
    public boolean z;
    public static byte B;
    public static short S;
    public static char C;
    public static boolean Z;

    enum Storage { ARRAY, INSTANCE, STATIC }

    static void check(MethodHandles.Lookup lookup) throws Throwable {
        for (var primitive : new Class<?>[]{byte.class, short.class, char.class, boolean.class}) {
            for (var storage : Storage.values()) check(lookup, primitive, storage);
        }
    }

    static void check(MethodHandles.Lookup lookup, Class<?> primitive, Storage storage) throws Throwable {
        var fieldName = primitive.descriptorString();
        var valueType = primitive.describeConstable().get();
        var owner = SubwordAtomics.class.describeConstable().get();
        var arrayType = valueType.arrayType();
        var argument = storage == Storage.ARRAY ? arrayType : owner;
        // Accept ints at the bytecode entry so javac cannot narrow them before the VarHandle call.
        var signature = MethodTypeDesc.of(CD_int, CD_VarHandle, argument, CD_int, CD_int);
        var coordinates = switch (storage) {
            case ARRAY -> new ClassDesc[]{arrayType, CD_int};
            case INSTANCE -> new ClassDesc[]{owner};
            case STATIC -> new ClassDesc[]{};
        };
        var methods = new String[]{"getAndSet", "compareAndExchange", "compareAndSet", "getAndBitwiseXor"};
        var caller = lookup.defineClass(ClassFile.of().build(ClassDesc.of("Raw" + fieldName + storage), b -> {
            for (String mode : methods) {
                b.withMethodBody(mode, signature, ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC, code -> {
                    code.aload(0);
                    if (storage != Storage.STATIC) code.aload(1);
                    if (storage == Storage.ARRAY) code.iconst_0();
                    var result = mode.equals("compareAndSet") ? CD_boolean : valueType;
                    var access = MethodTypeDesc.of(result, coordinates).insertParameterTypes(coordinates.length, valueType);
                    code.iload(2);
                    if (mode.startsWith("compare")) {
                        code.iload(3);
                        access = access.insertParameterTypes(access.parameterCount(), valueType);
                    }
                    code.invokevirtual(CD_VarHandle, mode, access).ireturn();
                });
            }
        }));
        var arrayClass = primitive.arrayType();
        Object receiver = storage == Storage.ARRAY ? java.lang.reflect.Array.newInstance(primitive, 1) : new SubwordAtomics();
        var handle = switch (storage) {
            case ARRAY -> MethodHandles.arrayElementVarHandle(arrayClass);
            case INSTANCE -> lookup.findVarHandle(SubwordAtomics.class, fieldName.toLowerCase(), primitive);
            case STATIC -> lookup.findStaticVarHandle(SubwordAtomics.class, fieldName, primitive);
        };
        var entry = MethodType.methodType(int.class, VarHandle.class,
                storage == Storage.ARRAY ? arrayClass : SubwordAtomics.class, int.class, int.class);
        int raw = 0x1fffa;
        int narrowed = primitive == boolean.class ? 1 : primitive == char.class ? (char) raw : (short) raw;
        // Unsafe compares a byte or short witness with the original JVM int for the CAS result.
        int casResult = primitive == byte.class || primitive == short.class ? 0 : 1;
        int[] expected = {0, narrowed, casResult, narrowed};
        for (int i = 0; i < methods.length; i++) {
            var call = lookup.findStatic(caller, methods[i], entry);
            int result = (int) call.invoke(handle, receiver, raw, raw);
            if (result != expected[i]) throw new AssertionError(fieldName + " " + storage + " " + methods[i] + ": " + result);
        }
        if ((int) lookup.findStatic(caller, "getAndSet", entry).invoke(handle, receiver, 0, 0) != 0)
            throw new AssertionError("XOR did not clear " + fieldName + " " + storage);
    }
}
