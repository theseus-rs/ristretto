public class Test {
    public static void main(String[] args) {
        testJsr();
    }

    static void testJsr() {
        // JSR/RET are deprecated bytecode instructions
        // Simulating subroutine behavior with method calls

        // Test basic subroutine call
        int result1 = subroutine1(5);
        System.out.println(result1);

        // Test subroutine with different parameter
        int result2 = subroutine1(10);
        System.out.println(result2);

        // Test nested subroutine calls
        int result3 = subroutine2(3);
        System.out.println(result3);

        // Test subroutine with zero parameter
        int result4 = subroutine1(0);
        System.out.println(result4);

        // Test subroutine with negative parameter
        int result5 = subroutine1(-5);
        System.out.println(result5);
    }

    static int subroutine1(int x) {
        return x * 2;
    }

    static int subroutine2(int x) {
        int temp = subroutine1(x);
        return temp + 1;
    }
}
