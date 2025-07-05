/** Test int array operations. */
public class Test {
    public static void main(String[] args) {
        int[] array = new int[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = 100;
        System.out.println(array[1]);
        array[0] = array[1] + 1;
        System.out.println(array[0]);
        try {
            int v = array[-1];
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
