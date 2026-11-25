/** Separate class: Program must reject the unimplemented pop2 opcode. */
public final class CanonicalUnsupportedPop2 {
    static long produce(int value) { return value; }
    public static int run(int value) { produce(value); return value; }
}
