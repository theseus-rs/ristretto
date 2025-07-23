/** Test default methods in interfaces and conflict resolution */
interface DefaultMethodA {
    default void sharedMethod() {
        System.out.println("DefaultMethodA.sharedMethod");
    }

    default void uniqueA() {
        System.out.println("DefaultMethodA.uniqueA");
    }
}

interface DefaultMethodB {
    default void sharedMethod() {
        System.out.println("DefaultMethodB.sharedMethod");
    }

    default void uniqueB() {
        System.out.println("DefaultMethodB.uniqueB");
    }
}

// This class must override sharedMethod due to conflict
class ConflictResolver implements DefaultMethodA, DefaultMethodB {
    @Override
    public void sharedMethod() {
        System.out.println("ConflictResolver.sharedMethod - resolving conflict");
        DefaultMethodA.super.sharedMethod();
        DefaultMethodB.super.sharedMethod();
    }
}

class DefaultMethodUser implements DefaultMethodA {
    // Uses default implementation of sharedMethod and uniqueA
}

interface ParentWithDefault {
    default void parentMethod() {
        System.out.println("ParentWithDefault.parentMethod");
    }
}

interface ChildOverrides extends ParentWithDefault {
    @Override
    default void parentMethod() {
        System.out.println("ChildOverrides.parentMethod - overridden");
    }
}

class InheritsOverriddenDefault implements ChildOverrides {
    // Gets the overridden version from ChildOverrides
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Default Methods Test ===");

        // Test conflict resolution
        ConflictResolver resolver = new ConflictResolver();
        resolver.sharedMethod();
        resolver.uniqueA();
        resolver.uniqueB();

        // Test default method usage
        DefaultMethodUser user = new DefaultMethodUser();
        user.sharedMethod();
        user.uniqueA();

        // Test default method inheritance override
        InheritsOverriddenDefault inherited = new InheritsOverriddenDefault();
        inherited.parentMethod();

        // Test via interface references
        DefaultMethodA refA = resolver;
        DefaultMethodB refB = resolver;

        System.out.println("Via DefaultMethodA reference:");
        refA.sharedMethod();
        refA.uniqueA();

        System.out.println("Via DefaultMethodB reference:");
        refB.sharedMethod();
        refB.uniqueB();
    }
}

