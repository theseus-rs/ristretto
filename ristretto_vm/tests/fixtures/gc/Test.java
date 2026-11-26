import java.lang.ref.*;

public class Test {
    static volatile int finalized;
    static volatile Object resurrected;
    static ReferenceQueue<Object> queue = new ReferenceQueue<Object>();
    static WeakReference<Object> weak;
    static PhantomReference<Object> phantom;
    static volatile boolean referenceOwnerFinalized;
    static volatile boolean referenceOwnerCleared;
    static volatile Object referenceOwnerTarget;

    static class FinalizableTarget {
        @Override protected void finalize() { referenceOwnerTarget = this; }
    }
    static class FinalizableWeakReference extends WeakReference<Object> {
        FinalizableWeakReference(Object target) { super(target); }
        @Override protected void finalize() {
            referenceOwnerCleared = get() == null;
            referenceOwnerFinalized = true;
        }
    }
    public static void allocateFinalizableReference() {
        new FinalizableWeakReference(new FinalizableTarget());
    }

    public static void allocate() {
        Object object = new Test();
        weak = new WeakReference<Object>(object, queue);
        phantom = new PhantomReference<Object>(object, queue);
    }
    @Override protected void finalize() {
        finalized++;
        resurrected = this;
    }
    public static void release() { resurrected = null; }
    public static boolean catchOom() {
        Object[] live = new Object[1024];
        try {
            for (int i = 0; i < live.length; i++) live[i] = new byte[65536];
            return false;
        } catch (OutOfMemoryError expected) {
            return true;
        }
    }
    public static int poll() {
        Reference<?> reference = queue.poll();
        if (reference == weak) return 1;
        if (reference == phantom) return 2;
        return 0;
    }
    static class CloneableValue implements Cloneable {
        int scalar = 7;
        Object link = new Object();
        CloneableValue duplicate() throws CloneNotSupportedException {
            return (CloneableValue) super.clone();
        }
    }
    public static boolean cloneCopies() throws CloneNotSupportedException {
        byte[] original = new byte[] { 1, 2, 3 };
        byte[] copy = original.clone();
        if (copy == original || copy[1] != 2 || copy.length != 3) return false;
        copy[0] = 9;
        if (original[0] != 1) return false;
        Object[] references = new Object[] { original };
        Object[] copiedReferences = references.clone();
        if (copiedReferences == references || copiedReferences[0] != original) return false;
        copiedReferences[0] = copy;
        if (references[0] != original) return false;
        CloneableValue value = new CloneableValue();
        CloneableValue cloned = value.duplicate();
        if (cloned == value || cloned.scalar != 7 || cloned.link != value.link) return false;
        cloned.scalar = 9;
        return value.scalar == 7;
    }
    public static boolean cloneOom() {
        byte[] original = new byte[9 * 1024 * 1024];
        try {
            original.clone();
            return false;
        } catch (OutOfMemoryError expected) {
            return original.length == 9 * 1024 * 1024;
        }
    }
}
