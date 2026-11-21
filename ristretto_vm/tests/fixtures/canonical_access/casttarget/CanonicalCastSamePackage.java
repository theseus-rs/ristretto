package casttarget;
public class CanonicalCastSamePackage {
    public static int classCast(int value) { Object object = null; return (CanonicalCastAccessTarget) object == null ? value : -1; }
    public static int arrayCast(int value) { Object object = null; return (CanonicalCastAccessTarget[]) object == null ? value : -1; }
    public static int arrayNew(int value) { return new CanonicalCastAccessTarget[value].length; }
    public static int objectNew(int value) { Object object = new CanonicalCastAccessTarget(); return object == null ? -1 : value; }
}
