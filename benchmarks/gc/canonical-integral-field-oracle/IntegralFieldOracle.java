import java.lang.classfile.ClassFile;
import java.lang.classfile.attribute.ConstantValueAttribute;
import java.lang.constant.ClassDesc;
import java.lang.constant.MethodTypeDesc;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

import static java.lang.classfile.ClassFile.*;
import static java.lang.constant.ConstantDescs.*;

/**
 * Generate Java 17 bytecode with the JDK 25 class-file API. Raw int operands
 * make the field instruction itself responsible for narrowing each store.
 * Run with java -Xverify:all IntegralFieldOracle.java OUTPUT_DIRECTORY.
 */
public class IntegralFieldOracle extends ClassLoader {
    private static final String NAME = "CanonicalIntegralFields";
    private static final ClassDesc OWNER = ClassDesc.of(NAME);
    private static final ClassDesc[] TYPES = {CD_boolean, CD_byte, CD_char, CD_short};
    private static final MethodTypeDesc ENTRY = MethodTypeDesc.of(CD_int, CD_int);
    private static final int CONSTANT = 0x180fe;
    private static final int[] INPUTS = {
        Integer.MIN_VALUE, -65537, -129, -1, 0, 1, 2, 127, 128, 255, 32768,
        65535, 65536, Integer.MAX_VALUE
    };

    public static void main(String[] args) throws ReflectiveOperationException, java.io.IOException {
        Path output = Path.of(args[0]);
        byte[] bytes = ClassFile.of().build(OWNER, cls -> {
            cls.withVersion(61, 0).withFlags(ACC_PUBLIC | ACC_SUPER);
            cls.withMethodBody("<init>", MTD_void, ACC_PUBLIC, code ->
                code.aload(0).invokespecial(CD_Object, "<init>", MTD_void).return_());
            for (ClassDesc type : TYPES) {
                String suffix = type.descriptorString();
                String instance = "value" + suffix;
                String shared = "shared" + suffix;
                String constant = "constant" + suffix;
                cls.withField(instance, type, 0);
                cls.withField(shared, type, ACC_STATIC);
                cls.withField(constant, type, field -> field.withFlags(ACC_STATIC | ACC_FINAL)
                    .with(ConstantValueAttribute.of(CONSTANT)));
                cls.withMethodBody("instance" + suffix, ENTRY, ACC_PUBLIC | ACC_STATIC, code ->
                    code.new_(OWNER).dup().invokespecial(OWNER, "<init>", MTD_void)
                        .dup().iload(0).putfield(OWNER, instance, type)
                        .getfield(OWNER, instance, type).ireturn());
                cls.withMethodBody("static" + suffix, ENTRY, ACC_PUBLIC | ACC_STATIC, code ->
                    code.iload(0).putstatic(OWNER, shared, type)
                        .getstatic(OWNER, shared, type).ireturn());
                cls.withMethodBody(constant, ENTRY, ACC_PUBLIC | ACC_STATIC, code ->
                    code.getstatic(OWNER, constant, type).ireturn());
            }
        });
        Class<?> oracle = new IntegralFieldOracle().defineClass(NAME, bytes, 0, bytes.length);
        StringBuilder rows = new StringBuilder("# method\targument\tHotSpot result\n");
        for (ClassDesc type : TYPES) {
            for (String prefix : new String[]{"instance", "static", "constant"}) {
                String name = prefix + type.descriptorString();
                var method = oracle.getMethod(name, int.class);
                for (int input : prefix.equals("constant") ? new int[]{0} : INPUTS) {
                    rows.append(name).append('\t').append(input).append('\t')
                        .append(method.invoke(null, input)).append('\n');
                }
            }
        }
        Files.write(output.resolve(NAME + ".class"), bytes);
        Files.writeString(output.resolve("integral-field-oracle.tsv"), rows, StandardCharsets.UTF_8);
    }
}
