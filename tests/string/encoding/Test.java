public class Test {
    public static final String EMPTY = "";
    public static final String SINGLE_A = "A";
    public static final String ASCII_PRINTABLE = "Hello, World! 1234567890";
    public static final String ASCII_CONTROL = "Tab:\t Newline:\n CarriageReturn:\r";
    public static final String ASCII_BOUNDARY_LOW = "\u0000"; // Null
    public static final String ASCII_BOUNDARY_HIGH = "\u007F"; // DEL

    public static final String LATIN1_MIN = "\u00A1"; // In Latin1 range, inverted exclamation
    public static final String LATIN1_MAX = "\u00FF"; // Latin small letter y with diaeresis

    public static final String BMP_MIN = "\u0100"; // Start of BMP after Latin1
    public static final String BMP_MAX = "\uFFFF"; // Last BMP code point

    public static final String NON_BMP_MIN = "\uD800\uDC00"; // First supplementary char (U+10000)
    public static final String NON_BMP_MAX = "\uDBFF\uDFFF"; // Last valid surrogate pair (U+10FFFF)

    public static final String EMOJI = "\uD83D\uDE03 \uD83D\uDC4D"; // 😃 👍

    public static final String CHINESE = "中文字符"; // Chinese characters
    public static final String ARABIC = "العربية"; // Arabic text
    public static final String HEBREW = "עברית"; // Hebrew text

    public static final String COMBINING = "e\u0301 = é"; // e + acute = é

    public static final String BIDIRECTIONAL = "English עברית العربية English"; // Mix of LTR/RTL
    public static final String EMBEDDED_NULL = "ABC\u0000DEF";

    public static void main(String[] args) {
        System.out.print("EMPTY: ");
        System.out.println(EMPTY);

        System.out.print("SINGLE_A: ");
        System.out.println(SINGLE_A);

        System.out.print("ASCII_PRINTABLE: ");
        System.out.println(ASCII_PRINTABLE);

        System.out.print("ASCII_CONTROL: ");
        System.out.println(ASCII_CONTROL);

        System.out.print("ASCII_BOUNDARY_LOW (null): ");
        System.out.println(ASCII_BOUNDARY_LOW);

        System.out.print("ASCII_BOUNDARY_HIGH (DEL): ");
        System.out.println(ASCII_BOUNDARY_HIGH);

        System.out.print("LATIN1_MIN (¡): ");
        System.out.println(LATIN1_MIN);

        System.out.print("LATIN1_MAX (ÿ): ");
        System.out.println(LATIN1_MAX);

        System.out.print("BMP_MIN (Ā): ");
        System.out.println(BMP_MIN);

        System.out.print("BMP_MAX: ");
        System.out.println(BMP_MAX);

        System.out.print("NON_BMP_MIN (U+10000): ");
        System.out.println(NON_BMP_MIN);

        System.out.print("NON_BMP_MAX (U+10FFFF): ");
        System.out.println(NON_BMP_MAX);

        System.out.print("EMOJI: ");
        System.out.println(EMOJI);

        System.out.print("CHINESE: ");
        System.out.println(CHINESE);

        System.out.print("ARABIC: ");
        System.out.println(ARABIC);

        System.out.print("HEBREW: ");
        System.out.println(HEBREW);

        System.out.print("COMBINING: ");
        System.out.println(COMBINING);

        System.out.print("BIDIRECTIONAL: ");
        System.out.println(BIDIRECTIONAL);

        System.out.print("EMBEDDED_NULL: ");
        System.out.println(EMBEDDED_NULL);
    }
}
