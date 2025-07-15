/** Test method visibility and access modifiers. */
public class Test {
    static class Parent {
        public void publicMethod() {
            System.out.println("Parent public method");
        }

        protected void protectedMethod() {
            System.out.println("Parent protected method");
        }

        void packageMethod() {
            System.out.println("Parent package method");
        }

        private void privateMethod() {
            System.out.println("Parent private method");
        }

        public void callPrivateMethod() {
            privateMethod();
        }
    }

    static class Child extends Parent {
        @Override
        public void publicMethod() {
            System.out.println("Child public method");
        }

        @Override
        protected void protectedMethod() {
            System.out.println("Child protected method");
        }

        @Override
        void packageMethod() {
            System.out.println("Child package method");
        }

        // Cannot override private method - this is a new method
        private void privateMethod() {
            System.out.println("Child private method (not override)");
        }

        public void testAccess() {
            publicMethod();
            protectedMethod();
            packageMethod();
            privateMethod(); // Calls Child's private method
            super.callPrivateMethod(); // Calls Parent's private method via public method
        }
    }

    public static void main(String[] args) {
        Child child = new Child();
        child.testAccess();

        Parent parent = new Child();
        parent.publicMethod();
        parent.protectedMethod();
        parent.packageMethod();
        parent.callPrivateMethod();
    }
}

