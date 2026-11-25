package foreign;

public class Visible {
    public static int initializations;
    public static class PublicArgument {
        static { initializations++; }
    }
}
class HiddenArgument {
    static { Visible.initializations++; }
}
