public final class ExperimentalJitDriver {
    public static void main(String[] args) {
        System.out.println(CanonicalGraphWorkload.run(Integer.parseInt(args[0])));
        System.out.println(ExperimentalJitLifecycle.catchNull());
    }
}
