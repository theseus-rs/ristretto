/** Test byte array operations. */
public class Test {
    public static void main(String[] args) {
        byte[] array = new byte[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = 42;
        System.out.println(array[1]);
        array[0] = (byte)(array[1] + 1);
        System.out.println(array[0]);
        try {
            byte v = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = 10;
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
