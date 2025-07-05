/** Test double array operations. */
public class Test {
    public static void main(String[] args) {
        double[] array = new double[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = 3.14;
        System.out.println(array[1]);
        array[0] = array[1] + 1.0;
        System.out.println(array[0]);
        try {
            double v = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = 2.71;
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
