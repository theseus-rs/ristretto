import java.util.Iterator;
import java.util.function.Function;
import java.util.stream.Stream;

/** Exercises the VM-backed StackTraceElement initialization paths. */
public class Test {
    public static void main(String[] args) {
        testThrowableStackTraceElement();
        testBootstrapStackTraceElement();
        testStackWalkerStackTraceElement();
    }

    /** Throwable.getStackTrace() uses the bulk StackTraceElement initializer. */
    private static void testThrowableStackTraceElement() {
        System.out.println("=== Throwable StackTraceElement ===");
        StackTraceElement element = createThrowable().getStackTrace()[0];

        System.out.println("class name: " + "Test".equals(element.getClassName()));
        System.out.println("method name: " + "createThrowable".equals(element.getMethodName()));
        System.out.println("file name: " + "Test.java".equals(element.getFileName()));
        System.out.println("source line: " + (element.getLineNumber() > 0));
        System.out.println("native method: " + element.isNativeMethod());
        System.out.println("class loader: " + "app".equals(element.getClassLoaderName()));
        System.out.println("module name null: " + (element.getModuleName() == null));
        System.out.println("module version null: " + (element.getModuleVersion() == null));
        System.out.println("formatted class: " + element.toString().contains("Test.createThrowable"));
    }

    private static Throwable createThrowable() {
        return new Throwable("integration test");
    }

    /** A Java runtime frame carries bootstrap-loader and named-module metadata. */
    private static void testBootstrapStackTraceElement() {
        System.out.println("=== Bootstrap StackTraceElement ===");
        try {
            Integer.parseInt("not-an-integer");
            throw new AssertionError("integer parsing unexpectedly succeeded");
        } catch (NumberFormatException exception) {
            StackTraceElement element = findFrame(
                exception.getStackTrace(),
                "java.lang.Integer",
                "parseInt"
            );

            System.out.println("frame found: " + (element != null));
            if (element == null) {
                return;
            }
            System.out.println("file name: " + "Integer.java".equals(element.getFileName()));
            System.out.println("source line: " + (element.getLineNumber() > 0));
            System.out.println("native method: " + element.isNativeMethod());
            System.out.println("class loader null: " + (element.getClassLoaderName() == null));
            System.out.println("module name: " + "java.base".equals(element.getModuleName()));
            System.out.println("module version present: " + (element.getModuleVersion() != null));
            String formatted = element.toString();
            System.out.println(
                "formatted module: "
                    + (formatted.contains("java.base")
                        && formatted.contains("/java.lang.Integer.parseInt"))
            );
        }
    }

    /** StackFrame.toStackTraceElement() uses the single-element initializer. */
    private static void testStackWalkerStackTraceElement() {
        System.out.println("=== StackWalker StackTraceElement ===");
        StackTraceElement element = captureStackWalkerElement();

        System.out.println("class name: " + "Test".equals(element.getClassName()));
        System.out.println(
            "method name: " + "captureStackWalkerElement".equals(element.getMethodName())
        );
        System.out.println("file name: " + "Test.java".equals(element.getFileName()));
        System.out.println("source line: " + (element.getLineNumber() > 0));
        System.out.println("native method: " + element.isNativeMethod());
        System.out.println("class loader: " + "app".equals(element.getClassLoaderName()));
        System.out.println("module name null: " + (element.getModuleName() == null));
        System.out.println("module version null: " + (element.getModuleVersion() == null));
        System.out.println(
            "formatted class: " + element.toString().contains("Test.captureStackWalkerElement")
        );
    }

    private static StackTraceElement captureStackWalkerElement() {
        final StackWalker walker = StackWalker.getInstance(
            StackWalker.Option.RETAIN_CLASS_REFERENCE
        );
        return walker.walk(
            new Function<Stream<StackWalker.StackFrame>, StackTraceElement>() {
                @Override
                public StackTraceElement apply(Stream<StackWalker.StackFrame> stream) {
                    Iterator<StackWalker.StackFrame> frames = stream.iterator();
                    while (frames.hasNext()) {
                        StackWalker.StackFrame frame = frames.next();
                        if (frame.getClassName().equals("Test")
                            && frame.getMethodName().equals("captureStackWalkerElement")) {
                            return frame.toStackTraceElement();
                        }
                    }
                    throw new AssertionError("captureStackWalkerElement frame not found");
                }
            }
        );
    }

    private static StackTraceElement findFrame(
        StackTraceElement[] elements,
        String className,
        String methodName
    ) {
        for (StackTraceElement element : elements) {
            if (element.getClassName().equals(className)
                && element.getMethodName().equals(methodName)) {
                return element;
            }
        }
        return null;
    }
}
