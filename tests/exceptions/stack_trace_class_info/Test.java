/**
 * Test that StackTraceElement contains correct class information.
 *
 * When an exception is thrown, the JVM populates StackTraceElement objects
 * via the native initStackTraceElements method. Each element must have:
 * - declaringClass (String): the fully qualified class name
 * - declaringClassObject (Class): the actual Class mirror
 * - methodName (String): the method name
 * - fileName (String): the source file name
 * - lineNumber (int): the line number
 *
 * The declaringClassObject field is used by StackTraceElement.computeFormat()
 * to determine the class loader and module information for formatting.
 */
public class Test {
    public static void main(String[] args) {
        testStackTraceHasClassName();
        testStackTraceHasMethodName();
        testStackTraceHasFileName();
        testStackTraceHasLineNumber();
        testStackTraceToString();
        testNestedExceptionStackTrace();
    }

    /** Stack trace elements must contain the correct declaring class name. */
    static void testStackTraceHasClassName() {
        System.out.println("=== Stack Trace Class Name ===");
        try {
            throwException();
        } catch (Exception e) {
            StackTraceElement[] trace = e.getStackTrace();
            if (trace.length > 0) {
                String className = trace[0].getClassName();
                System.out.println("Class name: " + className);
                System.out.println("Is Test: " + "Test".equals(className));
            } else {
                System.out.println("ERROR: Empty stack trace");
            }
        }
    }

    /** Stack trace elements must contain the correct method name. */
    static void testStackTraceHasMethodName() {
        System.out.println("\n=== Stack Trace Method Name ===");
        try {
            throwException();
        } catch (Exception e) {
            StackTraceElement[] trace = e.getStackTrace();
            if (trace.length > 0) {
                String methodName = trace[0].getMethodName();
                System.out.println("Method name: " + methodName);
                System.out.println("Is throwException: " + "throwException".equals(methodName));
            } else {
                System.out.println("ERROR: Empty stack trace");
            }
        }
    }

    /** Stack trace elements must contain the source file name. */
    static void testStackTraceHasFileName() {
        System.out.println("\n=== Stack Trace File Name ===");
        try {
            throwException();
        } catch (Exception e) {
            StackTraceElement[] trace = e.getStackTrace();
            if (trace.length > 0) {
                String fileName = trace[0].getFileName();
                System.out.println("File name: " + fileName);
                System.out.println("Is Test.java: " + "Test.java".equals(fileName));
            } else {
                System.out.println("ERROR: Empty stack trace");
            }
        }
    }

    /** Stack trace elements must contain a positive line number. */
    static void testStackTraceHasLineNumber() {
        System.out.println("\n=== Stack Trace Line Number ===");
        try {
            throwException();
        } catch (Exception e) {
            StackTraceElement[] trace = e.getStackTrace();
            if (trace.length > 0) {
                int lineNumber = trace[0].getLineNumber();
                System.out.println("Has line number: " + (lineNumber > 0));
            } else {
                System.out.println("ERROR: Empty stack trace");
            }
        }
    }

    /**
     * StackTraceElement.toString() exercises computeFormat() which requires
     * the declaringClassObject field to determine module/classloader info.
     * If declaringClassObject is null, computeFormat() throws NPE.
     */
    static void testStackTraceToString() {
        System.out.println("\n=== Stack Trace toString ===");
        try {
            throwException();
        } catch (Exception e) {
            StackTraceElement[] trace = e.getStackTrace();
            if (trace.length > 0) {
                String str = trace[0].toString();
                System.out.println("toString contains class: " + str.contains("Test"));
                System.out.println("toString contains method: " + str.contains("throwException"));
            } else {
                System.out.println("ERROR: Empty stack trace");
            }
        }
    }

    /** Nested exceptions should also have valid stack traces. */
    static void testNestedExceptionStackTrace() {
        System.out.println("\n=== Nested Exception Stack Trace ===");
        try {
            throwNestedException();
        } catch (Exception e) {
            // Check outer exception
            StackTraceElement[] trace = e.getStackTrace();
            boolean outerOk = trace.length > 0 && trace[0].getClassName().equals("Test");
            System.out.println("Outer trace has class: " + outerOk);

            // Check cause
            Throwable cause = e.getCause();
            if (cause != null) {
                StackTraceElement[] causeTrace = cause.getStackTrace();
                boolean causeOk = causeTrace.length > 0
                    && causeTrace[0].getClassName().equals("Test");
                System.out.println("Cause trace has class: " + causeOk);
                // toString on cause's stack trace should not throw NPE
                String causeStr = causeTrace[0].toString();
                System.out.println("Cause toString ok: " + (causeStr != null));
            } else {
                System.out.println("ERROR: No cause found");
            }
        }
    }

    private static void throwException() throws Exception {
        throw new RuntimeException("test exception");
    }

    private static void throwNestedException() throws Exception {
        try {
            throwException();
        } catch (Exception e) {
            throw new RuntimeException("wrapper", e);
        }
    }
}
