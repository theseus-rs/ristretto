public class Test {
    public static void main(String[] args) {
        testLookupSwitch();
    }

    static void testLookupSwitch() {
        // Test lookupswitch with sparse values
        testSparseSwitchValue(10);
        testSparseSwitchValue(50);
        testSparseSwitchValue(100);
        testSparseSwitchValue(500);
        testSparseSwitchValue(1000);

        // Test lookupswitch with values not in cases
        testSparseSwitchValue(0);
        testSparseSwitchValue(25);
        testSparseSwitchValue(75);
        testSparseSwitchValue(999);

        // Test lookupswitch with negative values
        testSparseSwitchValue(-10);
        testSparseSwitchValue(-50);
        testSparseSwitchValue(-100);

        // Test lookupswitch with boundary values
        testSparseSwitchValue(Integer.MIN_VALUE);
        testSparseSwitchValue(Integer.MAX_VALUE);
    }

    static void testSparseSwitchValue(int value) {
        switch (value) {
            case 10:
                System.out.println(10);
                break;
            case 50:
                System.out.println(50);
                break;
            case 100:
                System.out.println(100);
                break;
            case 500:
                System.out.println(500);
                break;
            case 1000:
                System.out.println(1000);
                break;
            default:
                System.out.println(-1);
                break;
        }
    }
}
