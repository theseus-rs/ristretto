/** A private main(String[]) is not a candidate and must not hide a valid main(). */
class Test {
    private static void main(String[] args) {
        throw new AssertionError("private main must not be launched");
    }

    public void main() {
        System.out.println("non-private main");
    }
}
