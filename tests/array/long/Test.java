/** Test long array operations. */
public class Test {
    public static void main(String[] args) {
        long[] array = new long[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = 123456789L;
        System.out.println(array[1]);
        array[0] = array[1] + 1;
        System.out.println(array[0]);
        try {
            long v = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = 10L;
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
