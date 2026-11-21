public final class CanonicalStaticArrays {
    static int[] numbers;
    static Object[] objects;
    public static int store(int value) {
        numbers = new int[1];
        numbers[0] = value;
        objects = new Object[9];
        objects[8] = numbers;
        return ((int[]) objects[8])[0];
    }
    public static int unlink(int unused) {
        int[] saved = (int[]) objects[8];
        numbers = null;
        objects = null;
        return saved[0];
    }
}
