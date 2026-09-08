import clojure.lang.*;
import java.io.StringReader;

public final class ClojureScript {
    public static void check(String source) throws Exception {
        // Syntax checking must not evaluate #= or resolve user namespace aliases by executing ns.
        LispReader.Resolver resolver = new LispReader.Resolver() {
            public Symbol currentNS() { return Symbol.intern("user"); }
            public Symbol resolveClass(Symbol symbol) { return null; }
            public Symbol resolveAlias(Symbol symbol) { return symbol; }
            public Symbol resolveVar(Symbol symbol) { return symbol; }
        };
        Var.pushThreadBindings(RT.map(RT.READEVAL, Boolean.FALSE, RT.var("clojure.core", "*reader-resolver*"), resolver));
        try {
            LineNumberingPushbackReader reader = new LineNumberingPushbackReader(new StringReader(source));
            Object eof = new Object();
            while (LispReader.read(reader, false, eof, false) != eof) { }
        } finally {
            Var.popThreadBindings();
        }
    }

    public static void run(String source) throws Exception {
        // Initialize the runtime before the compiler, whose static fields depend on it.
        RT.init();
        clojure.lang.Compiler.load(new StringReader(source), "Main.clj", "Main.clj");
    }
}
