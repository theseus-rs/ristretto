/** Separate class: Program must reject the unimplemented dup2 opcode. */
public final class CanonicalUnsupportedDup2 {
    public static int run(int value) { long wide = value; return (int) wide++; }
}
