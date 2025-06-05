use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/invoke/DirectMethodHandle$Holder";

/// Register all intrinsic methods for `java.lang.invoke.DirectMethodHandle$Holder`.
///
/// These methods are used for direct method invocation and field access in the JVM.  The methods
/// are not defined in the class file, but are provided by the JVM as intrinsic methods.  These
/// methods can be discovered by running the following code in the JVM:
///
/// ```java
/// import java.lang.invoke.MethodType;
/// import java.lang.reflect.Method;
/// import java.util.ArrayList;
/// import java.util.Comparator;
/// import java.util.List;
///
/// public class ListDirectMethodHandleHolderMethods {
///     public static void main(String[] args) throws Exception {
///         System.out.println("Java version: " + System.getProperty("java.version"));
///         List<String> signatures = new ArrayList<>();
///         Class<?> clazz = Class.forName("java.lang.invoke.DirectMethodHandle$Holder");
///         Method[] methods = clazz.getDeclaredMethods();
///         for (Method method : methods) {
///             MethodType mt = MethodType.methodType(method.getReturnType(), method.getParameterTypes());
///             signatures.add(method.getName() + mt.toMethodDescriptorString());
///         }
///         signatures.sort(Comparator.naturalOrder());
///         for (String signature : signatures) {
///             System.out.println(signature);
///         }
///     }
/// }
/// ```
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() == JAVA_11 {
        registry.register(
            CLASS_NAME,
            "getObject",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            get_object_0,
        );
        registry.register(
            CLASS_NAME,
            "getObject",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            get_object_1,
        );
        registry.register(
            CLASS_NAME,
            "getObjectVolatile",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            get_object_volatile_0,
        );
        registry.register(
            CLASS_NAME,
            "getObjectVolatile",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            get_object_volatile_1,
        );
        registry.register(
            CLASS_NAME,
            "invokeInterface",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            invoke_interface_0,
        );
        registry.register(
            CLASS_NAME,
            "invokeInterface",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
            invoke_interface_2,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;",
            invoke_special_3,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;II)I",
            invoke_special_6,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;III)I",
            invoke_special_8,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;III)Ljava/lang/Object;",
            invoke_special_9,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IIII)I",
            invoke_special_10,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;)I",
            invoke_special_11,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;Ljava/lang/Object;)I",
            invoke_special_12,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)I",
            invoke_special_13,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;",
            invoke_special_14,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)I",
            invoke_special_15,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            invoke_special_16,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)I",
            invoke_special_17,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecial",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I",
            invoke_special_25,
        );
        registry.register(
            CLASS_NAME,
            "invokeSpecialIFC",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I",
            invoke_special_ifc,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;)I",
            invoke_static_0,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;F)Ljava/lang/Object;",
            invoke_static_4,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            invoke_static_13,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
            invoke_static_28,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;",
            invoke_static_29,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;",
            invoke_static_30,
        );
        registry.register(
            CLASS_NAME,
            "invokeVirtual",
            "(Ljava/lang/Object;Ljava/lang/Object;I)I",
            invoke_virtual_2,
        );
        registry.register(
            CLASS_NAME,
            "putObject",
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            put_object_0,
        );
        registry.register(
            CLASS_NAME,
            "putObject",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
            put_object_1,
        );
        registry.register(
            CLASS_NAME,
            "putObjectVolatile",
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            put_object_volatile_0,
        );
        registry.register(
            CLASS_NAME,
            "putObjectVolatile",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
            put_object_volatile_1,
        );
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(CLASS_NAME, "invokeStatic", "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", invoke_static_33);
        registry.register(CLASS_NAME, "invokeStatic", "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", invoke_static_34);
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            invoke_static_55,
        );
        registry.register(
            CLASS_NAME,
            "invokeStatic",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            invoke_static_56,
        );
    }

    registry.register(
        CLASS_NAME,
        "getBoolean",
        "(Ljava/lang/Object;)I",
        get_boolean_0,
    );
    registry.register(
        CLASS_NAME,
        "getBoolean",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_boolean_1,
    );
    registry.register(
        CLASS_NAME,
        "getBooleanVolatile",
        "(Ljava/lang/Object;)I",
        get_boolean_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getBooleanVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_boolean_volatile_1,
    );
    registry.register(CLASS_NAME, "getByte", "(Ljava/lang/Object;)I", get_byte_0);
    registry.register(
        CLASS_NAME,
        "getByte",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_byte_1,
    );
    registry.register(
        CLASS_NAME,
        "getByteVolatile",
        "(Ljava/lang/Object;)I",
        get_byte_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getByteVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_byte_volatile_1,
    );
    registry.register(CLASS_NAME, "getChar", "(Ljava/lang/Object;)I", get_char_0);
    registry.register(
        CLASS_NAME,
        "getChar",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_char_1,
    );
    registry.register(
        CLASS_NAME,
        "getCharVolatile",
        "(Ljava/lang/Object;)I",
        get_char_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getCharVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_char_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "getDouble",
        "(Ljava/lang/Object;)D",
        get_double_0,
    );
    registry.register(
        CLASS_NAME,
        "getDouble",
        "(Ljava/lang/Object;Ljava/lang/Object;)D",
        get_double_1,
    );
    registry.register(
        CLASS_NAME,
        "getDoubleVolatile",
        "(Ljava/lang/Object;)D",
        get_double_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getDoubleVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)D",
        get_double_volatile_1,
    );
    registry.register(CLASS_NAME, "getFloat", "(Ljava/lang/Object;)F", get_float_0);
    registry.register(
        CLASS_NAME,
        "getFloat",
        "(Ljava/lang/Object;Ljava/lang/Object;)F",
        get_float_1,
    );
    registry.register(
        CLASS_NAME,
        "getFloatVolatile",
        "(Ljava/lang/Object;)F",
        get_float_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getFloatVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)F",
        get_float_volatile_1,
    );
    registry.register(CLASS_NAME, "getInt", "(Ljava/lang/Object;)I", get_int_0);
    registry.register(
        CLASS_NAME,
        "getInt",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_int_1,
    );
    registry.register(
        CLASS_NAME,
        "getIntVolatile",
        "(Ljava/lang/Object;)I",
        get_int_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getIntVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_int_volatile_1,
    );
    registry.register(CLASS_NAME, "getLong", "(Ljava/lang/Object;)J", get_long_0);
    registry.register(
        CLASS_NAME,
        "getLong",
        "(Ljava/lang/Object;Ljava/lang/Object;)J",
        get_long_1,
    );
    registry.register(
        CLASS_NAME,
        "getLongVolatile",
        "(Ljava/lang/Object;)J",
        get_long_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getLongVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)J",
        get_long_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "getReference",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        get_reference_0,
    );
    registry.register(
        CLASS_NAME,
        "getReference",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        get_reference_1,
    );
    registry.register(
        CLASS_NAME,
        "getReferenceVolatile",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        get_reference_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getReferenceVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        get_reference_volatile_1,
    );
    registry.register(CLASS_NAME, "getShort", "(Ljava/lang/Object;)I", get_short_0);
    registry.register(
        CLASS_NAME,
        "getShort",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_short_1,
    );
    registry.register(
        CLASS_NAME,
        "getShortVolatile",
        "(Ljava/lang/Object;)I",
        get_short_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "getShortVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        get_short_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeInterface",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_interface_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_special_0,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;",
        invoke_special_2,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;I)I",
        invoke_special_4,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_special_5,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;",
        invoke_special_7,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;",
        invoke_special_18,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;I)I",
        invoke_special_19,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;II)I",
        invoke_special_20,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I",
        invoke_special_21,
    );
    registry.register(CLASS_NAME, "invokeSpecial", "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)I", invoke_special_22);
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_special_23,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_24,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_special_26,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_27,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_28,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_29,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_30,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_31,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_32,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        invoke_special_33,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;I)J",
        invoke_special_34,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;J)J",
        invoke_special_35,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_special_36,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JI)J",
        invoke_special_37,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JJ)J",
        invoke_special_38,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JJ)Ljava/lang/Object;",
        invoke_special_39,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;)J",
        invoke_special_40,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;I)J",
        invoke_special_41,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J",
        invoke_special_42,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;J)J",
        invoke_special_43,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J",
        invoke_special_44,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_45,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_46,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_47,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_special_48,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        invoke_special_49,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;",
        invoke_special_50,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;",
        invoke_special_51,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;FLjava/lang/Object;)Ljava/lang/Object;",
        invoke_special_52,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;II)J",
        invoke_special_53,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)J",
        invoke_special_54,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;II)J",
        invoke_special_55,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JJ)J",
        invoke_special_56,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;I)J",
        invoke_special_57,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_58,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_59,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;",
        invoke_special_60,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;",
        invoke_special_61,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)J",
        invoke_special_62,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_63,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_64,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_special_65,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_66,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_67,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_68,
    );
    registry.register(
        CLASS_NAME,
        "invokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_special_69,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;)V",
        invoke_static_2,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;D)Ljava/lang/Object;",
        invoke_static_3,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;I)I",
        invoke_static_5,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;II)I",
        invoke_static_6,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;II)Ljava/lang/Object;",
        invoke_static_7,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;ILjava/lang/Object;)I",
        invoke_static_8,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;ILjava/lang/Object;II)I",
        invoke_static_9,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I",
        invoke_static_10,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_static_11,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_12,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_static_14,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_15,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_16,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_static_17,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_18,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        invoke_static_19,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I",
        invoke_static_20,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_21,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_22,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_23,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_24,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_25,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_26,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_27,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_31,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_32,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_static_35,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;",
        invoke_static_36,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_static_37,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;III)I",
        invoke_static_38,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_static_39,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JI)J",
        invoke_static_40,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JJ)J",
        invoke_static_41,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JLjava/lang/Object;)J",
        invoke_static_42,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J",
        invoke_static_43,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J",
        invoke_static_44,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J",
        invoke_static_45,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;I)I",
        invoke_static_46,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_static_47,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;",
        invoke_static_48,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;",
        invoke_static_49,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I",
        invoke_static_50,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;",
        invoke_static_51,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;",
        invoke_static_52,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_static_53,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        invoke_static_54,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;F)Ljava/lang/Object;",
        invoke_static_57,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_static_58,
    );
    registry.register(
        CLASS_NAME,
        "invokeStatic",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;",
        invoke_static_59,
    );
    registry.register(
        CLASS_NAME,
        "invokeStaticInit",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_init_0,
    );
    registry.register(
        CLASS_NAME,
        "invokeStaticInit",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_static_init_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeVirtual",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_virtual_0,
    );
    registry.register(
        CLASS_NAME,
        "invokeVirtual",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_virtual_1,
    );
    registry.register(
        CLASS_NAME,
        "invokeVirtual",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        invoke_virtual_3,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        new_invoke_special_0,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        new_invoke_special_1,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        new_invoke_special_2,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;II)Ljava/lang/Object;",
        new_invoke_special_3,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        new_invoke_special_4,
    );
    registry.register(
        CLASS_NAME,
        "newInvokeSpecial",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        new_invoke_special_5,
    );
    registry.register(
        CLASS_NAME,
        "putBoolean",
        "(Ljava/lang/Object;I)V",
        put_boolean_0,
    );
    registry.register(
        CLASS_NAME,
        "putBoolean",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_boolean_1,
    );
    registry.register(
        CLASS_NAME,
        "putBooleanVolatile",
        "(Ljava/lang/Object;I)V",
        put_boolean_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putBooleanVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_boolean_volatile_1,
    );
    registry.register(CLASS_NAME, "putByte", "(Ljava/lang/Object;I)V", put_byte_0);
    registry.register(
        CLASS_NAME,
        "putByte",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_byte_1,
    );
    registry.register(
        CLASS_NAME,
        "putByteVolatile",
        "(Ljava/lang/Object;I)V",
        put_byte_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putByteVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_byte_volatile_1,
    );
    registry.register(CLASS_NAME, "putChar", "(Ljava/lang/Object;I)V", put_char_0);
    registry.register(
        CLASS_NAME,
        "putChar",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_char_1,
    );
    registry.register(
        CLASS_NAME,
        "putCharVolatile",
        "(Ljava/lang/Object;I)V",
        put_char_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putCharVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_char_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "putDouble",
        "(Ljava/lang/Object;D)V",
        put_double_0,
    );
    registry.register(
        CLASS_NAME,
        "putDouble",
        "(Ljava/lang/Object;Ljava/lang/Object;D)V",
        put_double_1,
    );
    registry.register(
        CLASS_NAME,
        "putDoubleVolatile",
        "(Ljava/lang/Object;D)V",
        put_double_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putDoubleVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;D)V",
        put_double_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "putFloat",
        "(Ljava/lang/Object;F)V",
        put_float_0,
    );
    registry.register(
        CLASS_NAME,
        "putFloat",
        "(Ljava/lang/Object;Ljava/lang/Object;F)V",
        put_float_1,
    );
    registry.register(
        CLASS_NAME,
        "putFloatVolatile",
        "(Ljava/lang/Object;F)V",
        put_float_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putFloatVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;F)V",
        put_float_volatile_1,
    );
    registry.register(CLASS_NAME, "putInt", "(Ljava/lang/Object;I)V", put_int_0);
    registry.register(
        CLASS_NAME,
        "putInt",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_int_1,
    );
    registry.register(
        CLASS_NAME,
        "putIntVolatile",
        "(Ljava/lang/Object;I)V",
        put_int_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putIntVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_int_volatile_1,
    );
    registry.register(CLASS_NAME, "putLong", "(Ljava/lang/Object;J)V", put_long_0);
    registry.register(
        CLASS_NAME,
        "putLong",
        "(Ljava/lang/Object;Ljava/lang/Object;J)V",
        put_long_1,
    );
    registry.register(
        CLASS_NAME,
        "putLongVolatile",
        "(Ljava/lang/Object;J)V",
        put_long_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putLongVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;J)V",
        put_long_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "putReference",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        put_reference_0,
    );
    registry.register(
        CLASS_NAME,
        "putReference",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        put_reference_1,
    );
    registry.register(
        CLASS_NAME,
        "putReferenceVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        put_reference_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putReferenceVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        put_reference_volatile_1,
    );
    registry.register(
        CLASS_NAME,
        "putShort",
        "(Ljava/lang/Object;I)V",
        put_short_0,
    );
    registry.register(
        CLASS_NAME,
        "putShort",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_short_1,
    );
    registry.register(
        CLASS_NAME,
        "putShortVolatile",
        "(Ljava/lang/Object;I)V",
        put_short_volatile_0,
    );
    registry.register(
        CLASS_NAME,
        "putShortVolatile",
        "(Ljava/lang/Object;Ljava/lang/Object;I)V",
        put_short_volatile_1,
    );
}

#[async_recursion(?Send)]
async fn get_boolean_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_boolean_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_boolean_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_boolean_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_byte_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_byte_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_byte_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_byte_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_char_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_char_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_char_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_char_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_double_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;)D")
}

#[async_recursion(?Send)]
async fn get_double_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;Ljava/lang/Object;)D"
    )
}

#[async_recursion(?Send)]
async fn get_double_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;)D")
}

#[async_recursion(?Send)]
async fn get_double_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;)D"
    )
}

#[async_recursion(?Send)]
async fn get_float_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;)F")
}

#[async_recursion(?Send)]
async fn get_float_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;Ljava/lang/Object;)F"
    )
}

#[async_recursion(?Send)]
async fn get_float_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;)F")
}

#[async_recursion(?Send)]
async fn get_float_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;)F"
    )
}

#[async_recursion(?Send)]
async fn get_int_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_int_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_int_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_int_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_long_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn get_long_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;Ljava/lang/Object;)J"
    )
}

#[async_recursion(?Send)]
async fn get_long_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn get_long_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;Ljava/lang/Object;)J"
    )
}

#[async_recursion(?Send)]
async fn get_object_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_object_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_object_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_object_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_reference_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_reference_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_reference_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_reference_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn get_short_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_short_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn get_short_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_short_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_interface_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_interface_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_interface_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_2(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_3(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_4(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_5(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_6(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_7(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_8(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;III)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_9(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;III)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_10(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIII)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_11(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_12(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_13(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_14(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_15(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_16(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_17(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_18(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_19(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;I)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_20(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;II)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_21(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_22(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_23(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_24(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_25(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_26(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_27(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_28(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_29(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_30(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_31(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_32(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_33(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn invoke_special_34(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_35(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;J)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_36(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_37(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JI)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_38(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JJ)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_39(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JJ)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_40(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_41(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;I)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_42(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_43(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;J)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_44(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_45(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_46(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_47(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_48(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_49(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_50(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_51(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_52(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;FLjava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_53(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_54(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_55(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;II)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_56(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JJ)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_57(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;I)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_58(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_59(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_60(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_61(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_62(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_63(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_64(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_65(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_66(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_67(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_68(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_69(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_special_ifc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeSpecialIFC(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn invoke_static_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_2(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn invoke_static_3(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;D)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_4(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;F)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_5(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;I)I")
}

#[async_recursion(?Send)]
async fn invoke_static_6(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;II)I")
}

#[async_recursion(?Send)]
async fn invoke_static_7(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;II)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_8(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_9(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;II)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_10(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_11(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_12(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_13(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_14(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_15(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_16(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_17(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_18(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_19(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_20(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_21(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_22(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_23(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_24(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_25(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_26(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_27(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_28(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_29(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_30(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_31(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_32(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_33(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_34(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_35(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_36(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_37(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;I)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_38(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;III)I");
}
#[async_recursion(?Send)]
async fn invoke_static_39(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_40(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JI)J");
}
#[async_recursion(?Send)]
async fn invoke_static_41(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JJ)J");
}
#[async_recursion(?Send)]
async fn invoke_static_42(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;)J"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_43(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_44(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_45(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_46(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;I)I"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_47(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_48(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_49(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_50(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_51(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_52(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_53(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}
#[async_recursion(?Send)]
async fn invoke_static_54(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_55(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_56(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_57(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;F)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_58(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_59(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    );
}

#[async_recursion(?Send)]
async fn invoke_static_init_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStaticInit(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_static_init_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeStaticInit(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_virtual_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_virtual_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn invoke_virtual_2(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;I)I"
    )
}

#[async_recursion(?Send)]
async fn invoke_virtual_3(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;I)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_3(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;II)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_4(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn new_invoke_special_5(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[async_recursion(?Send)]
async fn put_boolean_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_boolean_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_boolean_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_boolean_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_byte_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_byte_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_byte_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_byte_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_char_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_char_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_char_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_char_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_double_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;D)V")
}

#[async_recursion(?Send)]
async fn put_double_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )
}

#[async_recursion(?Send)]
async fn put_double_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;D)V")
}

#[async_recursion(?Send)]
async fn put_double_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )
}

#[async_recursion(?Send)]
async fn put_float_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;F)V")
}

#[async_recursion(?Send)]
async fn put_float_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )
}

#[async_recursion(?Send)]
async fn put_float_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;F)V")
}

#[async_recursion(?Send)]
async fn put_float_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )
}

#[async_recursion(?Send)]
async fn put_int_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_int_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_int_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_int_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_long_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;J)V")
}

#[async_recursion(?Send)]
async fn put_long_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )
}

#[async_recursion(?Send)]
async fn put_long_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;J)V")
}

#[async_recursion(?Send)]
async fn put_long_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )
}

#[async_recursion(?Send)]
async fn put_object_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_object_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_object_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_object_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_reference_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_reference_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_reference_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_reference_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn put_short_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_short_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[async_recursion(?Send)]
async fn put_short_volatile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;I)V")
}

#[async_recursion(?Send)]
async fn put_short_volatile_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;)I"
    )]
    async fn test_get_char_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_char_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_char_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;)D"
    )]
    async fn test_get_double_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;Ljava/lang/Object;)D"
    )]
    async fn test_get_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;)D"
    )]
    async fn test_get_double_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;)D"
    )]
    async fn test_get_double_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;)F"
    )]
    async fn test_get_float_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;Ljava/lang/Object;)F"
    )]
    async fn test_get_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;)F"
    )]
    async fn test_get_float_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;)F"
    )]
    async fn test_get_float_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;)I"
    )]
    async fn test_get_int_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_int_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_int_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;)J"
    )]
    async fn test_get_long_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_get_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;)J"
    )]
    async fn test_get_long_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_get_long_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;)I"
    )]
    async fn test_get_short_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_short_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_short_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_interface_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_interface_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_interface_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_interface_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeInterface(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_interface_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_interface_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_3(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_special_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_5() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_5(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)I"
    )]
    async fn test_invoke_special_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_7() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_7(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;III)I"
    )]
    async fn test_invoke_special_8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_8(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;III)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_9() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_9(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIII)I"
    )]
    async fn test_invoke_special_10() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_10(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;)I"
    )]
    async fn test_invoke_special_11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_11(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IIILjava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_12() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_12(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)I"
    )]
    async fn test_invoke_special_13() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_13(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_14() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_14(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_15() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_15(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_16(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)I"
    )]
    async fn test_invoke_special_17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_17(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_18() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_18(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;I)I"
    )]
    async fn test_invoke_special_19() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_19(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;II)I"
    )]
    async fn test_invoke_special_20() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_20(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I"
    )]
    async fn test_invoke_special_21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_21(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_22() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_22(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_23() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_23(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_24() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_24(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_special_25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_25(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_26() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_26(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_27() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_27(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_28() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_28(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_29() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_29(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_30() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_30(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_31() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_31(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_32(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_special_33() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_33(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;I)J"
    )]
    async fn test_invoke_special_34() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_34(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;J)J"
    )]
    async fn test_invoke_special_35() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_35(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_36() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_36(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JI)J"
    )]
    async fn test_invoke_special_37() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_37(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JJ)J"
    )]
    async fn test_invoke_special_38() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_38(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JJ)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_39() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_39(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;)J"
    )]
    async fn test_invoke_special_40() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_40(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;I)J"
    )]
    async fn test_invoke_special_41() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_41(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J"
    )]
    async fn test_invoke_special_42() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_42(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;J)J"
    )]
    async fn test_invoke_special_43() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_43(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J"
    )]
    async fn test_invoke_special_44() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_44(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_45() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_45(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_46() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_46(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_47() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_47(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_48() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_48(thread, Parameters::default()).await;
    }
    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_special_49() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_49(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_50() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_50(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_51() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_51(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;FLjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_52() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_52(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;II)J"
    )]
    async fn test_invoke_special_53() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_53(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)J"
    )]
    async fn test_invoke_special_54() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_54(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;II)J"
    )]
    async fn test_invoke_special_55() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_55(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;JJ)J"
    )]
    async fn test_invoke_special_56() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_56(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;I)J"
    )]
    async fn test_invoke_special_57() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_57(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_58() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_58(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_59() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_59(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_60() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_60(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;F)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_61() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_61(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)J"
    )]
    async fn test_invoke_special_62() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_62(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_63() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_63(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_64() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_64(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_special_65() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_65(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_66() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_66(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_67() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_67(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_68() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_68(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_special_69() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_69(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeSpecialIFC(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_special_ifc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_special_ifc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)I"
    )]
    async fn test_invoke_static_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;)V"
    )]
    async fn test_invoke_static_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;D)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_3(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;F)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_static_5() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_5(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;II)I"
    )]
    async fn test_invoke_static_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;II)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_7() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_7(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;)I"
    )]
    async fn test_invoke_static_8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_8(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;II)I"
    )]
    async fn test_invoke_static_9() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_9(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;ILjava/lang/Object;ILjava/lang/Object;)I"
    )]
    async fn test_invoke_static_10() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_10(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_static_11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_11(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_12() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_12(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_static_13() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_13(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_14() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_14(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_15() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_15(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_16() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_16(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_static_17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_17(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_18() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_18(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_static_19() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_19(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_invoke_static_20() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_20(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_21(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_22() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_22(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_23() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_23(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_24() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_24(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_25(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_26() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_26(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_27() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_27(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_28() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_28(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_29() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_29(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;IILjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_30() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_30(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_31() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_31(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_32() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_32(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_33() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_33(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_34() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_34(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_35() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_35(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;II)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_36() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_36(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_37() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_37(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;III)I"
    )]
    async fn test_invoke_static_38() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_38(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_39() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_39(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JI)J"
    )]
    async fn test_invoke_static_40() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_40(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JJ)J"
    )]
    async fn test_invoke_static_41() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_41(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;)J"
    )]
    async fn test_invoke_static_42() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_42(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;ILjava/lang/Object;)J"
    )]
    async fn test_invoke_static_43() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_43(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;JLjava/lang/Object;)J"
    )]
    async fn test_invoke_static_44() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_44(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_invoke_static_45() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_45(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_static_46() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_46(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_47() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_47(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;D)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_48() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_48(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;DLjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_49() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_49(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_static_50() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_50(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_51() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_51(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_52() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_52(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_53() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_53(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_static_54() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_54(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_55() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_55(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_56() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_56(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;F)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_57() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_57(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_58() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_58(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStatic(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;J)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_59() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_59(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStaticInit(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_init_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_init_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeStaticInit(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_static_init_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_static_init_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_virtual_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_virtual_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_virtual_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_virtual_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;I)I"
    )]
    async fn test_invoke_virtual_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_virtual_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.invokeVirtual(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_invoke_virtual_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_virtual_3(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;II)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_3(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.newInvokeSpecial(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_invoke_special_5() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_invoke_special_5(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_volatile_1(thread, Parameters::default()).await;
    }
}
