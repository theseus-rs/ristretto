public class Expressions {
    public static void main(String[] args) {
        if (true) {
            System.out.println(true);
        }
        if (1 == 1) {
            System.out.println(true);
        }
        if (2 > 1) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        int input = 3;
        // tableswitch
        switch (input) {
            case 3:
                System.out.println(3);
                break;
            case 4:
                System.out.println(4);
                break;
            case 5:
                System.out.println(5);
                break;
            default:
                System.out.println("default");
        }

        // lookupswitch
        switch (input) {
            case 3:
                System.out.println(3);
                break;
            case 5:
                System.out.println(5);
                break;
            default:
                System.out.println("default");
        }
    }

    public static int add(int a, int b) {
        return a + b;
    }
}
