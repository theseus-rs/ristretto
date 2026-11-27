import java.lang.reflect.Array;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Semantic oracle for exact exported allocation fixtures; no timing claims. */
public final class CheckAllocationContinuations {
    private static int checks;
    private static void check(boolean value) {
        if (!value) throw new AssertionError("check " + checks);
        checks++;
    }
    public static void main(String[] args) throws Exception {
        String[] names = {"booleans", "bytes", "chars", "shorts", "ints", "longs", "floats", "doubles"};
        Class<?>[] types = {boolean.class, byte.class, char.class, short.class, int.class, long.class, float.class, double.class};
        for (int i = 0; i < names.length; i++) {
            Method method = AllocationContinuations.class.getMethod(names[i], int.class);
            Object array = method.invoke(null, 3);
            Object zeroes = Array.newInstance(types[i], 3);
            check(array.getClass() == zeroes.getClass());
            check(Array.getLength(array) == 3);
            for (int j = 0; j < 3; j++) check(Array.get(array, j).equals(Array.get(zeroes, j)));
            check(Array.getLength(method.invoke(null, 0)) == 0);
            try {
                method.invoke(null, -1);
                throw new AssertionError("negative primitive size accepted");
            } catch (InvocationTargetException error) {
                check(error.getCause() instanceof NegativeArraySizeException);
            }
        }
        check(AllocationContinuations.object().getClass() == Object.class);
        Object[] refs = AllocationContinuations.references(3);
        check(refs.length == 3);
        for (Object value : refs) check(value == null);
        int[][] multi = AllocationContinuations.multi(3, 2);
        check(multi.length == 3);
        for (int[] row : multi) check(row.length == 2 && row[0] == 0 && row[1] == 0);
        check(multi[0] != multi[1]);
        int[][] partial = AllocationContinuations.partial(3);
        check(partial.length == 3);
        for (int[] row : partial) check(row == null);
        check(AllocationContinuations.caught(-1) == 1);
        check(AllocationContinuations.caught(0) == 0);
        int[] retained = {88};
        int attempts = AllocationContinuations.attempts;
        check(AllocationContinuations.retain(retained, 1024) == retained);
        check(AllocationContinuations.attempts == attempts + 1);
        try {
            AllocationContinuations.multi(0, -1);
            throw new AssertionError("negative inner size accepted after zero outer size");
        } catch (NegativeArraySizeException expected) { checks++; }
        try {
            AllocationContinuations.references(-1);
            throw new AssertionError("negative reference size accepted");
        } catch (NegativeArraySizeException expected) { checks++; }
        // This component's initializer spins forever. Successful array construction
        // proves it was not initialized; cancellation itself is a VM-specific test.
        AllocationInitialization[] array = AllocationInitializationCaller.array(2);
        check(array.length == 2 && array[0] == null && array[1] == null);
        AllocationInitialization[][] nested = AllocationInitializationCaller.multi(2, 3);
        check(nested.length == 2 && nested[0].length == 3 && nested[1].length == 3);
        check(nested[0] != nested[1]);
        System.out.println("allocation checks passed: " + checks);
    }
}
