import java.lang.ref.*;

/**
 * Tests for Reference equality and identity behavior.
 */
public class Test {

    public static void main(String[] args) {
        System.out.println("=== Reference Equality Tests ===");
        testReferenceEquality();
        testReferenceIdentity();
        testNullReferenceEquality();
        System.out.println("=== Reference Equality Tests Completed ===");
    }

    /**
     * Test reference equality with same and different objects
     */
    public static void testReferenceEquality() {
        System.out.println("\n--- Testing Reference Equality ---");

        Object obj1 = new Object();
        Object obj2 = new Object();

        WeakReference<Object> ref1a = new WeakReference<>(obj1);
        WeakReference<Object> ref1b = new WeakReference<>(obj1);
        WeakReference<Object> ref2 = new WeakReference<>(obj2);

        System.out.println("References to same object have same referent: " + (ref1a.get() == ref1b.get()));
        System.out.println("References to different objects have different referents: " + (ref1a.get() != ref2.get()));
        System.out.println("Reference objects are different instances: " + (ref1a != ref1b));

        // Test with different reference types
        SoftReference<Object> softRef = new SoftReference<>(obj1);
        PhantomReference<Object> phantomRef = new PhantomReference<>(obj1, new ReferenceQueue<>());

        System.out.println("WeakReference and SoftReference to same object: " + (ref1a.get() == softRef.get()));
        System.out.println("PhantomReference.get() is null: " + (phantomRef.get() == null));
        System.out.println("Different reference types are different instances: " + (ref1a.hashCode() != softRef.hashCode()));
    }

    /**
     * Test reference identity behavior
     */
    public static void testReferenceIdentity() {
        System.out.println("\n--- Testing Reference Identity ---");

        Object obj = new Object();
        WeakReference<Object> ref1 = new WeakReference<>(obj);
        WeakReference<Object> ref2 = new WeakReference<>(obj);

        // Test reference identity
        System.out.println("Same reference instance: " + (ref1 == ref1));
        System.out.println("Different reference instances: " + (ref1 != ref2));

        // Test referent identity
        System.out.println("Same referent identity: " + (ref1.get() == ref2.get()));
        System.out.println("Referent equals original object: " + (ref1.get() == obj));

        // Test after clearing one reference
        ref1.clear();
        System.out.println("After clearing ref1:");
        System.out.println("ref1.get() is null: " + (ref1.get() == null));
        System.out.println("ref2.get() still valid: " + (ref2.get() == obj));
        System.out.println("References still different instances: " + (ref1 != ref2));
    }

    /**
     * Test equality behavior with null references
     */
    public static void testNullReferenceEquality() {
        System.out.println("\n--- Testing Null Reference Equality ---");

        WeakReference<Object> nullRef1 = new WeakReference<>(null);
        WeakReference<Object> nullRef2 = new WeakReference<>(null);
        SoftReference<Object> nullSoftRef = new SoftReference<>(null);

        System.out.println("Null references return null: " + (nullRef1.get() == nullRef2.get()));
        System.out.println("Null reference objects are different instances: " + (nullRef1 != nullRef2));
        System.out.println("Different null reference types are different: " + (nullRef1.hashCode() != nullSoftRef.hashCode()));
        System.out.println("All null references return same null: " + (nullRef1.get() == nullSoftRef.get()));

        // Test comparison with cleared reference
        Object obj = new Object();
        WeakReference<Object> ref = new WeakReference<>(obj);
        ref.clear();

        System.out.println("Cleared reference equals null reference: " + (ref.get() == nullRef1.get()));
        System.out.println("But reference instances are different: " + (ref != nullRef1));
    }
}
