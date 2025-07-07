public class Test {
    public static void main(String[] args) {
        testTableSwitch();
    }

    static void testTableSwitch() {
        // Test tableswitch with consecutive values
        testSwitchValue(0);
        testSwitchValue(1);
        testSwitchValue(2);
        testSwitchValue(3);
        testSwitchValue(4);

        // Test tableswitch with out of range values
        testSwitchValue(-1);
        testSwitchValue(5);
        testSwitchValue(100);

        // Test tableswitch with boundary values
        testSwitchValue(Integer.MIN_VALUE);
        testSwitchValue(Integer.MAX_VALUE);
    }

    static void testSwitchValue(int value) {
        switch (value) {
            case 0:
                System.out.println(0);
                break;
            case 1:
                System.out.println(1);
                break;
            case 2:
                System.out.println(2);
                break;
            case 3:
                System.out.println(3);
                break;
            case 4:
                System.out.println(4);
                break;
            default:
                System.out.println(-1);
                break;
        }
    }
}
