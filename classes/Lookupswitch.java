public class Lookupswitch {
    public static void main(String[] stringArray) {
        printHello("Hello, world");
    }

    private static void printHello(String string) {
        switch (string) {
            case "Hello, world":
                System.out.println(string);
                return;
            case "test":
                System.out.println("test (this should not happen)");
                break;
            default:
                System.out.println("default branch (this should not happen)");
        }

        System.out.println("outside the switch-case statement (this should not happen)");
        System.out.println("https://github.com/theseus-rs/ristretto/issues/616");
    }
}
