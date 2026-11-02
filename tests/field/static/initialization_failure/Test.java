/** Tests initializer exception wrapping, Error propagation, and failed initialization caching. */
public class Test {
    static final RuntimeException EXCEPTION = new RuntimeException("initializer exception");
    static final Error ERROR = new AssertionError("initializer error");

    static class ThrowsException {
        static int value = failException();
    }

    static class ThrowsError {
        static int value = failError();
    }

    static class FailingParent {
        static int value = failException();
    }

    static class Child extends FailingParent {
        static int childValue = initializeChild();
    }

    static int failException() {
        throw EXCEPTION;
    }

    static int failError() {
        throw ERROR;
    }

    static int initializeChild() {
        System.out.println("child initializer should not execute");
        return 1;
    }

    public static void main(String[] args) {
        for (int attempt = 0; attempt < 2; attempt++) {
            try {
                System.out.println(ThrowsException.value);
            } catch (ExceptionInInitializerError error) {
                System.out.println("exception cause preserved: " + (error.getCause() == EXCEPTION));
            } catch (NoClassDefFoundError error) {
                System.out.println("exception failure cached");
            }

            try {
                System.out.println(ThrowsError.value);
            } catch (NoClassDefFoundError error) {
                System.out.println("error failure cached");
            } catch (Error error) {
                System.out.println("error propagated unchanged: " + (error == ERROR));
            }

            try {
                System.out.println(Child.childValue);
            } catch (ExceptionInInitializerError error) {
                System.out.println("parent cause preserved: " + (error.getCause() == EXCEPTION));
            } catch (NoClassDefFoundError error) {
                System.out.println("child failure cached");
            }
        }
    }
}
