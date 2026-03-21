/** Test interface method resolution through super-interface hierarchies.
 *
 * This exercises JVMS §5.4.3.4 which specifies that when resolving an interface
 * method, the JVM should check java.lang.Object for public instance methods and
 * search parent interfaces for the method.
 */
public class Test {
    interface Base {
        default String getName() {
            return "Base";
        }
    }

    interface Middle extends Base {
        default String getMiddleInfo() {
            return "Middle";
        }
    }

    interface Leaf extends Middle {
        default String getLeafInfo() {
            return "Leaf";
        }
    }

    // Diamond inheritance through interfaces
    interface Left extends Base {
        default String getSide() {
            return "Left";
        }
    }

    interface Right extends Base {
        default String getSide() {
            return "Right";
        }
    }

    // Deep hierarchy with method defined only at the top
    interface Level1 {
        default String deepMethod() {
            return "Level1";
        }
    }

    interface Level2 extends Level1 {
    }

    interface Level3 extends Level2 {
    }

    interface Level4 extends Level3 {
    }

    static class LeafImpl implements Leaf {
    }

    static class DiamondImpl implements Left, Right {
        @Override
        public String getSide() {
            return "DiamondResolved";
        }
    }

    static class DeepImpl implements Level4 {
    }

    // Test interface with method that overrides at an intermediate level
    interface OverrideBase {
        default String value() {
            return "OverrideBase";
        }
    }

    interface OverrideMiddle extends OverrideBase {
        @Override
        default String value() {
            return "OverrideMiddle";
        }
    }

    interface OverrideLeaf extends OverrideMiddle {
    }

    static class OverrideImpl implements OverrideLeaf {
    }

    public static void main(String[] args) {
        System.out.println("=== Super-Interface Method Resolution ===");

        // Test linear hierarchy resolution
        LeafImpl leaf = new LeafImpl();
        System.out.println("getName (from Base): " + leaf.getName());
        System.out.println("getMiddleInfo (from Middle): " + leaf.getMiddleInfo());
        System.out.println("getLeafInfo (from Leaf): " + leaf.getLeafInfo());

        // Access via different interface references
        Leaf leafRef = leaf;
        Middle middleRef = leaf;
        Base baseRef = leaf;
        System.out.println("via Leaf ref - getName: " + leafRef.getName());
        System.out.println("via Middle ref - getName: " + middleRef.getName());
        System.out.println("via Base ref - getName: " + baseRef.getName());

        // Test Object methods through deep interface hierarchy
        System.out.println("via Leaf ref - toString not null: " + (leafRef.toString() != null));
        System.out.println("via Leaf ref - hashCode works: " + (leafRef.hashCode() != -1));
        System.out.println("via Leaf ref - equals self: " + leafRef.equals(leaf));

        // Test diamond resolution
        DiamondImpl diamond = new DiamondImpl();
        System.out.println("diamond getSide: " + diamond.getSide());
        System.out.println("diamond getName (from Base): " + diamond.getName());

        Left leftRef = diamond;
        Right rightRef = diamond;
        System.out.println("via Left ref - getSide: " + leftRef.getSide());
        System.out.println("via Right ref - getSide: " + rightRef.getSide());

        // Test deep hierarchy (method defined 4 levels up)
        DeepImpl deep = new DeepImpl();
        System.out.println("deep deepMethod: " + deep.deepMethod());

        Level4 l4 = deep;
        Level3 l3 = deep;
        Level2 l2 = deep;
        Level1 l1 = deep;
        System.out.println("via Level4 - deepMethod: " + l4.deepMethod());
        System.out.println("via Level3 - deepMethod: " + l3.deepMethod());
        System.out.println("via Level2 - deepMethod: " + l2.deepMethod());
        System.out.println("via Level1 - deepMethod: " + l1.deepMethod());

        // Test Object methods via deep interface ref
        System.out.println("via Level4 - equals self: " + l4.equals(deep));
        System.out.println("via Level4 - toString not null: " + (l4.toString() != null));

        // Test override at intermediate level
        OverrideImpl overrideImpl = new OverrideImpl();
        System.out.println("override value: " + overrideImpl.value());
        OverrideLeaf overrideLeaf = overrideImpl;
        OverrideMiddle overrideMiddle = overrideImpl;
        OverrideBase overrideBase = overrideImpl;
        System.out.println("via OverrideLeaf - value: " + overrideLeaf.value());
        System.out.println("via OverrideMiddle - value: " + overrideMiddle.value());
        System.out.println("via OverrideBase - value: " + overrideBase.value());

        System.out.println("=== Super-Interface Method Resolution Complete ===");
    }
}
