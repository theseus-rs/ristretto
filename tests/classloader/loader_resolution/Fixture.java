public class Fixture {
    static { Test.initializations++; }
    public Object value;
    public static Object label = "parent";
    public static Object setLabel(Object next) { label = next; return null; }
    public static Object tag() { return label; }
    public static Object supplier() {
        java.util.function.Supplier<Object> get = Fixture::tag;
        return get.get();
    }
    public static Object single() { return new Fixture[1]; }
    public static Object multi() { return new Fixture[1][2]; }
    public static Object partial() { return new Fixture[1][2][]; }
    public static Object deferredArray() { return new Deferred[1]; }
    public static Object arrayLiteral() { return Deferred[][].class; }
    public static Object isArray(Object value) { return value instanceof Fixture[]; }
    public static Object castArray(Object value) { return (Fixture[]) value; }
    public static Object caught() {
        try { throw new ChildException(); }
        catch (ChildException e) { return "caught"; }
    }
    public static Object unmatchedCatch() {
        try { throw new RuntimeException(); }
        catch (ChildException e) { return "wrong catch"; }
        catch (RuntimeException e) { return "caught runtime"; }
    }
    public static Object param(Object value) { return value; }
    public static class Member {}
    public static class Deferred extends Fixture {
        static { Test.deferredInitializations++; }
    }
    public record Pair(Object value) {}
}
