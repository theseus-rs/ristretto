package fields;

import java.lang.invoke.VarHandle;

public class Target {
    public static int shared;
    public static Target peer;
    public int number;
    protected int inherited = 5;
    private int secret = 7;

    public int secret() { return secret; }

    public static Object readPeer(VarHandle field) { return (Target) field.get(); }
    public static void writePeer(VarHandle field, Object value) { field.set((Target) value); }
}
