/** Test for Fibonacci sequence computation. */
public class Test {
    // Computes fib(0..n) iteratively and returns all results.
    static long[] fibUpTo(int n) {
        long[] f = new long[n + 1];
        for (int i = 0; i <= n; i++) {
            if (i == 0) f[i] = 0L;
            else if (i == 1) f[i] = 1L;
            else f[i] = f[i - 1] + f[i - 2];
        }
        return f;
    }

    public static void main(String[] args) {
        int n = 50;
        long[] vals = fibUpTo(n);
        for (int i = 0; i <= n; i++) {
            System.out.println("fib(" + i + ") = " + vals[i]);
        }
    }
}
