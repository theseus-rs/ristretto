package q;

public class Child extends p.Base {
    // This is not an override of p.Base's package-private method.
    public String value() { return "child public"; }
}
