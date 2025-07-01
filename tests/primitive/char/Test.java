/** Test the char primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Character.MIN_HIGH_SURROGATE);
        System.out.println(Character.MAX_HIGH_SURROGATE);
        System.out.println(Character.MIN_LOW_SURROGATE);
        System.out.println(Character.MAX_LOW_SURROGATE);

        for (int c = 0; c < 4096; c++) {
            System.out.print(Character.toChars(c));
        }
    }
}
