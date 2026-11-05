public class Fixture {
    public Fixture marker;

    public Fixture(Fixture value) { marker = value; }

    public static Fixture identity(Fixture value, Fixture[] array, int primitive) {
        return value;
    }

    public static Object references() {
        Fixture value = new Fixture(null);
        value.marker = value;
        return identity(value, new Fixture[0], 1);
    }
}
