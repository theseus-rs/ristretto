import java.lang.classfile.ClassFile;
import java.lang.constant.ClassDesc;
import java.lang.constant.ConstantDescs;
import java.lang.constant.DirectMethodHandleDesc;
import java.lang.constant.DynamicConstantDesc;
import java.lang.constant.MethodHandleDesc;
import java.lang.constant.MethodTypeDesc;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.VarHandleDesc;

public class Test {
    int number = 7;
    static long total = 11L;

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        ClassDesc owner = Test.class.describeConstable().get();
        VarHandleDesc field = VarHandleDesc.ofField(owner, "number", ConstantDescs.CD_int);
        VarHandleDesc staticField = VarHandleDesc.ofStaticField(owner, "total", ConstantDescs.CD_long);
        VarHandleDesc array = VarHandleDesc.ofArray(ConstantDescs.CD_int.arrayType());
        MethodTypeDesc getter = MethodTypeDesc.of(ConstantDescs.CD_VarHandle);
        int flags = ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC;
        byte[] bytes = ClassFile.of().build(ClassDesc.of("HandleConstants"), builder -> {
            builder.withMethodBody("field", getter, flags, code -> code.ldc(field).areturn());
            var missing = VarHandleDesc.ofField(owner, "absent", ConstantDescs.CD_int);
            builder.withMethodBody("missing", getter, flags, code -> code.ldc(missing).areturn());
            builder.withMethodBody("staticField", getter, flags, code -> code.ldc(staticField).areturn());
            builder.withMethodBody("array", getter, flags, code -> code.ldc(array).areturn());
            var add = MethodHandleDesc.ofMethod(DirectMethodHandleDesc.Kind.VIRTUAL, ConstantDescs.CD_VarHandle,
                    "getAndAdd", MethodTypeDesc.of(ConstantDescs.CD_int, ConstantDescs.CD_int.arrayType(),
                            ConstantDescs.CD_int, ConstantDescs.CD_int));
            builder.withMethodBody("add", MethodTypeDesc.of(ConstantDescs.CD_MethodHandle), flags,
                    code -> code.ldc(add).areturn());
            var increment = DynamicConstantDesc.ofNamed(ConstantDescs.BSM_EXPLICIT_CAST, "increment", ConstantDescs.CD_int, 2);
            var total = DynamicConstantDesc.ofNamed(ConstantDescs.BSM_EXPLICIT_CAST, "total", ConstantDescs.CD_long, 0x1_0000_0011L);
            builder.withMethodBody("increment", MethodTypeDesc.of(ConstantDescs.CD_int), flags, code -> code.ldc(increment).ireturn());
            builder.withMethodBody("total", MethodTypeDesc.of(ConstantDescs.CD_long), flags, code -> code.ldc(total).lreturn());
        });
        Class<?> constants = lookup.defineClass(bytes);
        Test receiver = new Test();
        int[] values = {3};
        load(lookup, constants, "field").set(receiver, 13);
        long value = (long) lookup.findStatic(constants, "total", MethodType.methodType(long.class)).invokeExact();
        int delta = (int) lookup.findStatic(constants, "increment", MethodType.methodType(int.class)).invokeExact();
        load(lookup, constants, "staticField").set(value);
        int old = (int) load(lookup, constants, "array").getAndAdd(values, 0, delta);
        if (receiver.number != 13 || total != 0x1_0000_0011L || old != 3 || values[0] != 5) {
            throw new AssertionError("constant handle storage");
        }
        MethodHandle add = (MethodHandle) lookup.findStatic(constants, "add", MethodType.methodType(MethodHandle.class))
                .invokeExact();
        VarHandle element = load(lookup, constants, "array");
        if ((int) add.invokeExact(element, values, 0, 7) != 5 || values[0] != 12) {
            throw new AssertionError("ldc method handle for VarHandle.getAndAdd");
        }
        MethodHandle missing = lookup.findStatic(constants, "missing", MethodType.methodType(VarHandle.class));
        for (int attempt = 0; attempt < 2; attempt++) {
            try {
                VarHandle unexpected = (VarHandle) missing.invokeExact();
                throw new AssertionError("ldc resolved a missing field to " + unexpected);
            } catch (NoSuchFieldError expected) {
                // The compatibility runner compares the first and repeated linkage failures with OpenJDK.
                Throwable cause = expected.getCause();
                System.out.println("missing field ldc " + attempt + " cause " + (cause == null ? "none" : cause.getClass().getName()));
            }
        }
        booleanStores(lookup);
        SubwordAtomics.check(lookup);
        duplicateFieldNames(lookup);
        fixedResultDescriptors(lookup);
        missingCallSiteType(lookup);
        System.out.println("VarHandle constant linkage passed");
    }

    static void booleanStores(MethodHandles.Lookup lookup) throws Throwable {
        var booleans = ConstantDescs.CD_boolean.arrayType();
        var vh = ConstantDescs.CD_VarHandle;
        var signature = MethodTypeDesc.of(ConstantDescs.CD_int, vh, booleans, ConstantDescs.CD_int);
        var setType = MethodTypeDesc.of(ConstantDescs.CD_void, booleans, ConstantDescs.CD_int, ConstantDescs.CD_boolean);
        byte[] bytes = ClassFile.of().build(ClassDesc.of("BooleanArrayStore"), builder ->
                builder.withMethodBody("store", signature, ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC,
                        code -> code.aload(0).aload(1).iconst_0().iload(2).invokevirtual(vh, "set", setType)
                                .aload(1).iconst_0().baload().ireturn()));
        MethodHandle store = lookup.findStatic(lookup.defineClass(bytes), "store",
                MethodType.methodType(int.class, VarHandle.class, boolean[].class, int.class));
        VarHandle handle = MethodHandles.arrayElementVarHandle(boolean[].class);
        boolean[] values = new boolean[1];
        for (int value : new int[] {2, 3, -1, -2}) {
            if ((int) store.invokeExact(handle, values, value) != (value & 1)) throw new AssertionError("boolean array store " + value);
        }
    }

    static void missingCallSiteType(MethodHandles.Lookup lookup) throws Throwable {
        var missing = MethodTypeDesc.of(ClassDesc.of("MissingCallSiteResult"));
        var run = MethodTypeDesc.of(ConstantDescs.CD_void);
        byte[] bytes = ClassFile.of().build(ClassDesc.of("MissingCallSiteType"), builder -> {
            for (var name : new String[]{"get", "invoke", "invokeExact"}) {
                var owner = name.equals("get") ? ConstantDescs.CD_VarHandle : ConstantDescs.CD_MethodHandle;
                builder.withMethodBody(name, run, ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC,
                        code -> code.aconst_null().invokevirtual(owner, name, missing).pop().return_());
            }
        });
        Class<?> caller = lookup.defineClass(bytes);
        for (String name : new String[]{"get", "invoke", "invokeExact"}) {
            MethodHandle call = lookup.findStatic(caller, name, MethodType.methodType(void.class));
            try {
                call.invokeExact();
                throw new AssertionError("missing call-site type resolved for " + name);
            } catch (NoClassDefFoundError expected) {
            } catch (NullPointerException wrongOrder) {
                throw new AssertionError("null receiver checked before resolving " + name, wrongOrder);
            }
        }
    }

    static void fixedResultDescriptors(MethodHandles.Lookup lookup) throws Throwable {
        ClassDesc vh = ConstantDescs.CD_VarHandle, integer = ConstantDescs.CD_int;
        var signature = MethodTypeDesc.of(integer, vh);
        byte[] bytes = ClassFile.of().build(ClassDesc.of("FixedResults"), builder -> {
            for (String name : new String[]{"set", "setOpaque", "setRelease", "setVolatile", "compareAndSet", "weakCompareAndSetPlain"}) {
                builder.withMethodBody(name, signature, ClassFile.ACC_PUBLIC | ClassFile.ACC_STATIC, code -> {
                    code.aload(0).iconst_0();
                    var descriptor = MethodTypeDesc.of(integer, integer);
                    if (!name.startsWith("set")) {
                        code.iconst_1();
                        descriptor = MethodTypeDesc.of(integer, integer, integer);
                    }
                    code.invokevirtual(vh, name, descriptor).ireturn();
                });
            }
        });
        Class<?> caller = lookup.defineClass(bytes);
        VarHandle handle = lookup.findStaticVarHandle(Test.class, "total", long.class);
        long before = total;
        for (var method : caller.getDeclaredMethods()) {
            MethodHandle call = lookup.unreflect(method);
            for (VarHandle receiver : new VarHandle[]{handle, null}) {
                try {
                    int value = (int) call.invokeExact(receiver);
                    throw new AssertionError("linked " + method.getName());
                } catch (NoSuchMethodError expected) {}
            }
        }
        if (total != before) throw new AssertionError("invalid descriptor changed storage");
    }

    static void duplicateFieldNames(MethodHandles.Lookup lookup) throws Throwable {
        ClassDesc owner = ClassDesc.of("DuplicateFields");
        int pub = ClassFile.ACC_PUBLIC, statics = pub | ClassFile.ACC_STATIC;
        var init = MethodTypeDesc.of(ConstantDescs.CD_void);
        byte[] bytes = ClassFile.of().build(owner, builder -> {
            builder.withField("v", ConstantDescs.CD_int, statics);
            builder.withField("w", ConstantDescs.CD_long, pub);
            builder.withField("v", ConstantDescs.CD_long, statics);
            builder.withField("w", ConstantDescs.CD_int, pub);
            builder.withField("v", ConstantDescs.CD_Object, statics);
            builder.withField("v", ConstantDescs.CD_String, statics);
            builder.withMethodBody("<init>", init, pub, code -> code.aload(0)
                    .invokespecial(ConstantDescs.CD_Object, "<init>", init).return_());
            builder.withMethodBody("read", MethodTypeDesc.of(ConstantDescs.CD_long), pub, code -> code
                    .getstatic(owner, "v", ConstantDescs.CD_int).i2l()
                    .getstatic(owner, "v", ConstantDescs.CD_long).ladd()
                    .aload(0).getfield(owner, "w", ConstantDescs.CD_long).ladd()
                    .aload(0).getfield(owner, "w", ConstantDescs.CD_int).i2l().ladd().lreturn());
        });
        Class<?> cls = lookup.defineClass(bytes);
        Object instance = lookup.findConstructor(cls, MethodType.methodType(void.class)).invoke();
        VarHandle si = lookup.findStaticVarHandle(cls, "v", int.class);
        VarHandle sl = lookup.findStaticVarHandle(cls, "v", long.class);
        VarHandle ii = lookup.findVarHandle(cls, "w", int.class);
        VarHandle il = lookup.findVarHandle(cls, "w", long.class);
        VarHandle object = lookup.findStaticVarHandle(cls, "v", Object.class);
        VarHandle string = lookup.findStaticVarHandle(cls, "v", String.class);
        Object marker = new Object();
        object.set(marker);
        string.set("first");
        String old = (String) string.toMethodHandle(VarHandle.AccessMode.COMPARE_AND_EXCHANGE)
                .invokeExact("first", "second");
        if (old != "first" || object.get() != marker || string.get() != "second") throw new AssertionError("reference slots");
        si.set(13);
        sl.set(1L << 40);
        il.set(instance, 1L << 36);
        ii.set(instance, 7);
        if ((long) sl.toMethodHandle(VarHandle.AccessMode.COMPARE_AND_EXCHANGE)
                .invokeExact(1L << 40, 1L << 41) != (1L << 40)) throw new AssertionError("static exchange");
        if ((int) ii.toMethodHandle(VarHandle.AccessMode.GET).invoke(instance) != 7) throw new AssertionError("instance get");
        for (var field : cls.getDeclaredFields()) {
            if (field.getName().equals("v") && field.getType() == long.class
                    && (long) lookup.unreflectVarHandle(field).get() != (1L << 41)) {
                throw new AssertionError("reflected static long");
            }
            if (field.getName().equals("w") && field.getType() == int.class
                    && (int) lookup.unreflectGetter(field).invoke(instance) != 7) {
                throw new AssertionError("reflected instance int");
            }
        }
        if ((long) lookup.findVirtual(cls, "read", MethodType.methodType(long.class)).invoke(instance)
                != (1L << 41) + (1L << 36) + 20) throw new AssertionError("field slots");
    }

    static VarHandle load(MethodHandles.Lookup lookup, Class<?> owner, String name) throws Throwable {
        MethodHandle getter = lookup.findStatic(owner, name, MethodType.methodType(VarHandle.class));
        VarHandle first = (VarHandle) getter.invokeExact();
        if ((VarHandle) getter.invokeExact() != first) throw new AssertionError("unresolved constant " + name);
        return first;
    }
}
