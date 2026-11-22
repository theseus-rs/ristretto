public final class CanonicalScalarLoop {
    public static int calculate(int rounds) {
        int value = 1;
        for (int i = 0; i < rounds; i++) value = 31 * value - i;
        return value;
    }
}
class CanonicalScalarLoopOracle {
    public static void main(String[] args) {
        for (int rounds : new int[] {-1, 0, 1, 2, 8, 63})
            System.out.println(rounds + " " + CanonicalScalarLoop.calculate(rounds));
    }
}
