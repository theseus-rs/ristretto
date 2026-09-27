/** A Thread subclass must still execute Thread's private exception-dispatch callback. */
public class Test {
    static int handled;
    static class FailingThread extends Thread {
        public void run() { throw new IllegalStateException("failure"); }
    }
    static class Leaf extends FailingThread {}
    static class Handler implements Thread.UncaughtExceptionHandler {
        public void uncaughtException(Thread thread, Throwable failure) {
            if (failure instanceof IllegalStateException) handled++;
        }
    }
    static class InheritedHandler extends Handler {}

    public static void main(String[] args) throws Exception {
        Thread thread = new Leaf();
        thread.setUncaughtExceptionHandler(new InheritedHandler());
        thread.start();
        thread.join(5000);
        System.out.println("instance handler: " + handled);
        Thread.setDefaultUncaughtExceptionHandler(new InheritedHandler());
        try {
            thread = new Leaf();
            thread.start();
            thread.join(5000);
            System.out.println("default handler: " + handled);
        } finally {
            Thread.setDefaultUncaughtExceptionHandler(null);
        }
    }
}
