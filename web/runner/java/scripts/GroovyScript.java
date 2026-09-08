import groovy.lang.Binding;
import groovy.lang.GroovyClassLoader;
import groovy.lang.Script;
import org.codehaus.groovy.runtime.InvokerHelper;

public final class GroovyScript {
    private static Class<?> compile(String source) throws Exception {
        // Do not instantiate: script field initializers belong to Run, not Check.
        return new GroovyClassLoader(GroovyScript.class.getClassLoader()).parseClass(source, "Main.groovy");
    }

    public static void check(String source) throws Exception {
        compile(source);
    }

    public static void run(String source) throws Exception {
        Class<?> compiled = compile(source);
        Binding binding = new Binding();
        binding.setVariable("args", new String[0]);
        Script script = InvokerHelper.createScript(compiled, binding);
        script.run();
    }
}
