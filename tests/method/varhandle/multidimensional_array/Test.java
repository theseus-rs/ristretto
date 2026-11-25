import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.Arrays;

public class Test {
    public static void main(String[] args) {
        VarHandle rows = MethodHandles.arrayElementVarHandle(int[][].class);
        if (rows.varType() != int[].class || !rows.coordinateTypes().equals(Arrays.asList(int[][].class, int.class))) {
            throw new AssertionError("row handle type");
        }
        int[] original = {7};
        int[] replacement = {7};
        int[][] matrix = {null, original};
        if (rows.compareAndSet(matrix, 1, replacement, null)) throw new AssertionError("matched distinct row");
        if ((int[]) rows.getAndSet(matrix, 1, replacement) != original) throw new AssertionError("row exchange witness");
        if (matrix[0] != null || matrix[1] != replacement) throw new AssertionError("row storage");
        System.out.println("Multidimensional array access passed");
    }
}
