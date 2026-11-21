package castoutside;
import casttarget.CanonicalCastAccessTarget;
public class CanonicalCastOtherPackage {
    public static int classCast(int value) { Object object = new int[1]; return (CanonicalCastAccessTarget) object == null ? value : -1; }
    public static int arrayCast(int value) { Object object = new int[1]; return ((CanonicalCastAccessTarget[]) object).length; }
    public static int arrayNew(int value) { return new CanonicalCastAccessTarget[value].length; }
    public static int objectNew(int value) { Object object = new CanonicalCastAccessTarget(); return object == null ? -1 : value; }
}
