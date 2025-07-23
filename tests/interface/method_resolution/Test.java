/** Test interface method resolution and overriding */
interface BaseMethod {
    void commonMethod();
    default void defaultCommon() {
        System.out.println("BaseMethod.defaultCommon");
    }
}

interface OverrideMethodA extends BaseMethod {
    @Override
    default void defaultCommon() {
        System.out.println("OverrideMethodA.defaultCommon");
    }

    void specificA();
}

interface OverrideMethodB extends BaseMethod {
    @Override
    default void defaultCommon() {
        System.out.println("OverrideMethodB.defaultCommon");
    }

    void specificB();
}

// Class must resolve the conflict
class ConflictResolution implements OverrideMethodA, OverrideMethodB {
    @Override
    public void defaultCommon() {
        System.out.println("ConflictResolution.defaultCommon - manual resolution");
        OverrideMethodA.super.defaultCommon();
        OverrideMethodB.super.defaultCommon();
    }

    public void commonMethod() {
        System.out.println("ConflictResolution.commonMethod");
    }

    public void specificA() {
        System.out.println("ConflictResolution.specificA");
    }

    public void specificB() {
        System.out.println("ConflictResolution.specificB");
    }
}

// Test class hierarchy with interface methods
abstract class AbstractBase implements BaseMethod {
    public void commonMethod() {
        System.out.println("AbstractBase.commonMethod");
    }

    // Keep defaultCommon as default from interface

    abstract void abstractMethod();
}

class ConcreteChild extends AbstractBase implements OverrideMethodA {
    @Override
    public void defaultCommon() {
        System.out.println("ConcreteChild.defaultCommon - overriding default");
        super.defaultCommon(); // Calls interface default
    }

    public void specificA() {
        System.out.println("ConcreteChild.specificA");
    }

    void abstractMethod() {
        System.out.println("ConcreteChild.abstractMethod");
    }
}

// Test static method hiding
interface StaticHiding {
    static void staticMethod() {
        System.out.println("StaticHiding.staticMethod");
    }
}

interface StaticHidingChild extends StaticHiding {
    // This doesn't override but hides the static method
    static void staticMethod() {
        System.out.println("StaticHidingChild.staticMethod");
    }
}

class StaticUser implements StaticHidingChild {
    public static void classStaticMethod() {
        System.out.println("StaticUser.classStaticMethod");
    }
}

// Test covariant return types
interface CovariantBase {
    Object getValue();
    Number getNumber();
}

interface CovariantChild extends CovariantBase {
    @Override
    String getValue(); // Covariant return type

    @Override
    Integer getNumber(); // Covariant return type
}

class CovariantImpl implements CovariantChild {
    public String getValue() {
        return "Covariant String";
    }

    public Integer getNumber() {
        return 42;
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Method Resolution Test ===");

        // Test conflict resolution
        ConflictResolution conflict = new ConflictResolution();
        conflict.commonMethod();
        conflict.defaultCommon();
        conflict.specificA();
        conflict.specificB();

        // Test via different interface references
        OverrideMethodA aRef = conflict;
        OverrideMethodB bRef = conflict;
        BaseMethod baseRef = conflict;

        System.out.println("\nVia OverrideMethodA reference:");
        aRef.defaultCommon();
        aRef.specificA();

        System.out.println("Via OverrideMethodB reference:");
        bRef.defaultCommon();
        bRef.specificB();

        System.out.println("Via BaseMethod reference:");
        baseRef.defaultCommon();

        // Test inheritance with interfaces
        ConcreteChild child = new ConcreteChild();
        child.commonMethod();
        child.defaultCommon();
        child.specificA();
        child.abstractMethod();

        // Test static method hiding
        System.out.println("\nStatic method hiding tests:");
        StaticHiding.staticMethod();
        StaticHidingChild.staticMethod();
        StaticUser.classStaticMethod();

        // Test covariant returns
        CovariantImpl covariant = new CovariantImpl();
        Object obj = covariant.getValue();
        Number num = covariant.getNumber();
        String str = covariant.getValue(); // Direct call returns String
        Integer integer = covariant.getNumber(); // Direct call returns Integer

        System.out.println("\nCovariant return tests:");
        System.out.println("getValue as Object: " + obj + " (type: " + obj.getClass().getSimpleName() + ")");
        System.out.println("getNumber as Number: " + num + " (type: " + num.getClass().getSimpleName() + ")");
        System.out.println("getValue as String: " + str);
        System.out.println("getNumber as Integer: " + integer);

        // Test via base interface reference
        CovariantBase baseCovariant = covariant;
        Object baseObj = baseCovariant.getValue(); // Returns String but typed as Object
        Number baseNum = baseCovariant.getNumber(); // Returns Integer but typed as Number

        System.out.println("Via base interface - getValue: " + baseObj + " (type: " + baseObj.getClass().getSimpleName() + ")");
        System.out.println("Via base interface - getNumber: " + baseNum + " (type: " + baseNum.getClass().getSimpleName() + ")");

        // Test method resolution with multiple inheritance levels
        System.out.println("\nMethod resolution priority tests:");

        // Class method overrides interface default
        System.out.println("Child class overrides interface default:");
        child.defaultCommon();

        // Interface default used when class doesn't override
        AbstractBase abstractRef = child;
        System.out.println("Abstract class uses interface default (through child override):");
        abstractRef.defaultCommon();

        // Test that static methods are not inherited
        System.out.println("\nStatic method inheritance tests:");
        System.out.println("StaticUser does not inherit interface static methods");
        // StaticUser.staticMethod(); // This would be a compile error

        // Must call static methods on the declaring interface
        StaticHiding.staticMethod();
        StaticHidingChild.staticMethod();

        System.out.println("Method resolution tests completed");
    }
}
