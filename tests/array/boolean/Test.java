/** Test boolean array operations. */
public class Test {
    public static void main(String[] args) {
        boolean[] array = new boolean[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = true;
        System.out.println(array[1]);
        array[0] = !array[1];
        System.out.println(array[0]);
        try {
            boolean b = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = true;
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
