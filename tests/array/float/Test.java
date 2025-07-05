/** Test float array operations. */
public class Test {
    public static void main(String[] args) {
        float[] array = new float[2];
        System.out.println(array.length);
        System.out.println(array[0]);
        array[1] = 2.5f;
        System.out.println(array[1]);
        array[0] = array[1] + 1.0f;
        System.out.println(array[0]);
        try {
            float v = array[-1];
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
        try {
            array[-1] = 1.1f;
        } catch (ArrayIndexOutOfBoundsException error) {
            System.out.println(error);
        }
    }
}
