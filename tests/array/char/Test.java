/** Test char array operations. */
public class Test {
    public static void main(String[] args) {
        char[] array = new char[2];
        System.out.println(array.length);
        System.out.println((int)array[0]);
        array[1] = 'A';
        System.out.println(array[1]);
        array[0] = (char)(array[1] + 1);
        System.out.println(array[0]);
        try {
            char v = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = 'B';
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
