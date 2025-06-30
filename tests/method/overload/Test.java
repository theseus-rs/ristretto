public class Test {
    public void print(String message) {
        System.out.print("String: ");
        System.out.println(message);
    }

    public void print(int number) {
        System.out.print("int: ");
        System.out.println(number);
    }

    public void print(double number) {
        System.out.print("double: ");
        System.out.println(number);
    }

    public void print(char[] characters) {
        System.out.print("char[]: ");
        System.out.println(new String(characters));
    }

    public static void main(String[] args) {
        Test test = new Test();

        test.print("Hello, world!");
        test.print(42);
        test.print(3.14);
        test.print(new char[] {'J','a','v','a'});
    }
}
