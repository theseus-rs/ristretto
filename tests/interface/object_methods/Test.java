/** Test calling Object methods (toString, hashCode, equals) on interface references.
 *
 * This exercises the JVM specification requirement that interfaces implicitly extend
 * java.lang.Object, so methods like toString(), hashCode(), and equals() must be
 * resolvable when invoked on interface-typed references.
 */
public class Test {
    interface Describable {
        String describe();
    }

    interface Identifiable {
        int getId();
    }

    interface Combined extends Describable, Identifiable {
    }

    static class Entity implements Combined {
        private final int id;
        private final String name;

        Entity(int id, String name) {
            this.id = id;
            this.name = name;
        }

        @Override
        public String describe() {
            return "Entity(" + name + ")";
        }

        @Override
        public int getId() {
            return id;
        }

        @Override
        public String toString() {
            return "Entity{id=" + id + ", name='" + name + "'}";
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (!(obj instanceof Entity)) return false;
            Entity other = (Entity) obj;
            return id == other.id && name.equals(other.name);
        }

        @Override
        public int hashCode() {
            return id * 31 + name.hashCode();
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Object Methods on Interface References ===");

        Entity entity1 = new Entity(1, "Alice");
        Entity entity2 = new Entity(1, "Alice");
        Entity entity3 = new Entity(2, "Bob");

        // Call Object methods through interface references
        Describable desc = entity1;
        System.out.println("toString via Describable: " + desc.toString());
        System.out.println("hashCode via Describable: " + (desc.hashCode() != 0));
        System.out.println("equals via Describable: " + desc.equals(entity2));
        System.out.println("getClass via Describable: " + desc.getClass().getSimpleName());

        Identifiable ident = entity1;
        System.out.println("toString via Identifiable: " + ident.toString());
        System.out.println("equals via Identifiable (same): " + ident.equals(entity2));
        System.out.println("equals via Identifiable (diff): " + ident.equals(entity3));

        Combined combined = entity1;
        System.out.println("toString via Combined: " + combined.toString());
        System.out.println("equals via Combined: " + combined.equals(entity2));

        // Test that interface-specific methods still work alongside Object methods
        System.out.println("describe: " + desc.describe());
        System.out.println("getId: " + ident.getId());

        // Test with null comparison
        System.out.println("equals null: " + desc.equals(null));

        // Test identity equality through interface references
        Describable descRef1 = entity1;
        Describable descRef2 = entity1;
        System.out.println("identity equals: " + descRef1.equals(descRef2));

        // Test toString on interface reference with default toString
        Describable plain = new Describable() {
            @Override
            public String describe() {
                return "anonymous";
            }
        };
        // Anonymous class toString contains hash, just verify it's not null
        System.out.println("anonymous toString not null: " + (plain.toString() != null));
        System.out.println("anonymous describe: " + plain.describe());

        System.out.println("=== Object Methods on Interface References Complete ===");
    }
}
