/** Peer-only main/string output; never included in the admitted canonical classes. */
public final class CanonicalCompiledStaticOracle {
    public static void main(String[] arguments) {
        System.out.println("prepare\t0\t" + CanonicalCompiledStaticHandoff.prepare(0));
        System.out.println("store\t31\t" + CanonicalCompiledStaticHandoff.store(31));
        System.out.println("empty\t0\t" + CanonicalCompiledStaticHandoff.empty(0));
        System.out.println("store\t-7\t" + CanonicalCompiledStaticHandoff.store(-7));
        System.out.println("churn\t17\t" + CanonicalCompiledStaticHandoff.churn(17));
    }
}
