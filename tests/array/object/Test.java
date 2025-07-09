/** Test Object array operations. */
public class Test {
   public static void main(String[] args) {
       Object[] array = new Object[2];
       System.out.println(array.length);
       System.out.println(array[0]);
       array[0] = "Hello";
       System.out.println(array[0]);
       array[1] = "World";
       System.out.println(array[1]);
       try {
           Object v = array[-1];
       } catch (ArrayIndexOutOfBoundsException error) {
           System.out.println(error);
       }
       try {
           array[-1] = "Test";
       } catch (ArrayIndexOutOfBoundsException error) {
           System.out.println(error);
       }
   }
}
