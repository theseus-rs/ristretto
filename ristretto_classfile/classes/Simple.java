interface SimpleInterface {
}

@Deprecated
public class Simple implements SimpleInterface {
    public static final boolean BOOLEAN = true;
    public static final byte BYTE = Byte.MAX_VALUE;
    public static final char CHAR = 'A';
    public static final double DOUBLE = 6.2831855;
    public static final float FLOAT = 3.14159f;
    public static final int INTEGER = Integer.MAX_VALUE;
    public static final long LONG = Long.MAX_VALUE;
    public static final short SHORT = Short.MAX_VALUE;
    public static final String STRING = "foo";

    @Deprecated
    public int publicValue;
    protected int protectedValue;
    int defaultValue;
    private int privateValue;

    @Deprecated
    public Simple() {
        this.publicValue = 0;
        this.protectedValue = 1;
        this.defaultValue = 2;
        this.privateValue = 3;
    }

    @Deprecated
    public int getPublicValue() {
        return publicValue;
    }

    @Deprecated
    public void setPublicValue(int publicValue) {
        this.publicValue = publicValue;
    }

    protected int getProtectedValue() {
        return protectedValue;
    }

    int getDefaultValue() {
        return defaultValue;
    }

    private int getPrivateValue() {
        return privateValue;
    }

    public static void main(String[] args) {
        Simple simple = new Simple();
        System.out.println(simple.getPublicValue());
        System.out.println(simple.getProtectedValue());
        System.out.println(simple.getDefaultValue());
        System.out.println(simple.getPrivateValue());
    }
}
