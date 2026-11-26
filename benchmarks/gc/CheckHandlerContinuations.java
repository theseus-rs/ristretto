// Exact exported bytecode. Rust separately checks suspension and cancellation.
public final class CheckHandlerContinuations {
    private static int checks;
    private static void check(boolean value) {
        checks++;
        if (!value) throw new AssertionError("check " + checks);
    }
    public static void main(String[] args) {
        int[] array = HandlerEdgeLoop.run(3);
        check(array.length == 1);
        check(array[0] == 77);
        Throwable error = HandlerCall.run(false);
        check(error instanceof NullPointerException);
        check(error == HandlerCall.error);
        check(HandlerCall.array.length == 1);
        check(HandlerCall.array[0] == 0);
        System.out.println("PASS " + checks + " exact handler-continuation results");
    }
}
