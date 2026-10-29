import java.lang.ref.WeakReference;

public class Test {

    public static void main(String[] args) {
        Object value = new Object();
        WeakReference<Object> reference = new WeakReference<>(value);
        System.out.println("get returns referent: " + (reference.get() == value));
        reference.clear();
        System.out.println("get after clear is null: " + (reference.get() == null));
    }
}
