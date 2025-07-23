/** Test interface casting and type checking */
interface CastableA {
    void methodA();
    default void defaultA() {
        System.out.println("CastableA.defaultA");
    }
}

interface CastableB {
    void methodB();
    default void defaultB() {
        System.out.println("CastableB.defaultB");
    }
}

interface CastableC extends CastableA {
    void methodC();
}

class MultiCastImpl implements CastableA, CastableB {
    public void methodA() {
        System.out.println("MultiCastImpl.methodA");
    }

    public void methodB() {
        System.out.println("MultiCastImpl.methodB");
    }
}

class HierarchyCastImpl implements CastableC {
    public void methodA() {
        System.out.println("HierarchyCastImpl.methodA");
    }

    public void methodC() {
        System.out.println("HierarchyCastImpl.methodC");
    }
}

class OnlyAImpl implements CastableA {
    public void methodA() {
        System.out.println("OnlyAImpl.methodA");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Casting and Type Checking Test ===");

        // Test multiple interface casting
        MultiCastImpl multi = new MultiCastImpl();

        // Upcast to interfaces
        CastableA aRef = multi;
        CastableB bRef = multi;
        Object objRef = multi;

        System.out.println("Upcasting tests:");
        aRef.methodA();
        bRef.methodB();

        // Downcast back to concrete class
        if (aRef instanceof MultiCastImpl) {
            MultiCastImpl downcast = (MultiCastImpl) aRef;
            downcast.methodA();
            downcast.methodB();
            System.out.println("Downcast from CastableA successful");
        }

        // Cross-cast between interfaces
        if (aRef instanceof CastableB) {
            CastableB crossCast = (CastableB) aRef;
            crossCast.methodB();
            System.out.println("Cross-cast from CastableA to CastableB successful");
        }

        // Test hierarchy casting
        HierarchyCastImpl hierarchy = new HierarchyCastImpl();
        CastableC cRef = hierarchy;
        CastableA aFromC = hierarchy; // Implicit upcast through inheritance

        cRef.methodC();
        cRef.methodA(); // Inherited from CastableA
        aFromC.methodA();

        // Cast from child interface to parent interface
        CastableA aFromCCast = (CastableA) cRef;
        aFromCCast.methodA();
        System.out.println("Cast from CastableC to CastableA successful");

        // Test invalid casts
        OnlyAImpl onlyA = new OnlyAImpl();
        CastableA aOnlyRef = onlyA;

        System.out.println("\nInvalid cast tests:");
        System.out.println("aOnlyRef instanceof CastableB: " + (aOnlyRef instanceof CastableB));
        System.out.println("aOnlyRef instanceof CastableC: " + (aOnlyRef instanceof CastableC));

        // This would throw ClassCastException:
        try {
            CastableB invalidCast = (CastableB) aOnlyRef;
            System.out.println("Invalid cast succeeded - this shouldn't happen");
        } catch (ClassCastException e) {
            System.out.println("ClassCastException caught as expected: " + e.getMessage());
        }

        // Test null casting
        CastableA nullRef = null;
        System.out.println("\nNull casting tests:");
        System.out.println("null instanceof CastableA: " + (nullRef instanceof CastableA));
        System.out.println("null instanceof CastableB: " + (nullRef instanceof CastableB));

        // Casting null doesn't throw exception
        CastableB nullCast = (CastableB) nullRef;
        System.out.println("Null cast successful: " + (nullCast == null));

        // Test array casting
        System.out.println("\nArray casting tests:");
        CastableA[] aArray = {multi, hierarchy, onlyA};
        Object[] objArray = aArray; // Arrays are covariant

        System.out.println("aArray instanceof CastableA[]: " + (aArray instanceof CastableA[]));
        System.out.println("aArray instanceof Object[]: " + (aArray instanceof Object[]));
        System.out.println("objArray instanceof CastableA[]: " + (objArray instanceof CastableA[]));

        // Cast array back
        CastableA[] castBack = (CastableA[]) objArray;
        System.out.println("Array cast back successful");

        // Test with different array types
        MultiCastImpl[] multiArray = {multi};
        CastableA[] aFromMulti = multiArray; // Upcast array
        System.out.println("multiArray cast to CastableA[] successful");

        // Test complex casting scenarios
        System.out.println("\nComplex casting scenarios:");
        Object complexObj = multi;

        if (complexObj instanceof CastableA && complexObj instanceof CastableB) {
            System.out.println("Object implements both CastableA and CastableB");
            CastableA complexA = (CastableA) complexObj;
            CastableB complexB = (CastableB) complexObj;
            complexA.methodA();
            complexB.methodB();
        }

        // Test casting with generics
        java.util.List<CastableA> list = new java.util.ArrayList<>();
        list.add(multi);
        list.add(hierarchy);
        list.add(onlyA);

        for (CastableA item : list) {
            item.methodA();
            if (item instanceof CastableB) {
                ((CastableB) item).methodB();
            }
            if (item instanceof CastableC) {
                ((CastableC) item).methodC();
            }
        }

        System.out.println("Interface casting tests completed");
    }
}
