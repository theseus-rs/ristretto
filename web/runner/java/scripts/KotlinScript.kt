import java.io.File
import org.jetbrains.kotlin.KtNodeTypes
import org.jetbrains.kotlin.lexer.KtTokens
import kotlin.script.experimental.api.*
import kotlin.script.experimental.host.StringScriptSource
import kotlin.script.experimental.jvm.*
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost

/** Kept only for the lifetime of one playground worker. */
object KotlinScript {
    init {
        // Lexer tokens and PSI node types have circular static initializers. Initialize
        // tokens before the scripting host so qualified-access tokens enter the parser's sets.
        check(KtTokens.QUALIFIED_ACCESS.contains(KtNodeTypes.DOT_QUALIFIED_EXPRESSION)) {
            "Kotlin lexer token initialization failed"
        }
    }
    private val host = BasicJvmScriptingHost()
    private fun compile(source: String): CompiledScript {
        val jars = System.getProperty("java.class.path").split(File.pathSeparator).map(::File)
        val api = File(System.getProperty("ristretto.language.path", "."), "jdk-api.jar")
        val configuration = ScriptCompilationConfiguration {
            baseClass(kotlin.script.templates.standard.ScriptTemplateWithArgs::class)
            jvm {
                dependencies(JvmDependency(jars + api))
                jvmTarget("17")
            }
            compilerOptions("-Xbackend-threads=1", "-no-jdk")
        }
        return value(host.runInCoroutineContext {
            host.compiler(StringScriptSource(source, "Main.kts"), configuration)
        })
    }

    @JvmStatic
    fun check(source: String) {
        compile(source)
    }

    @JvmStatic
    fun run(source: String) {
        val compiled = compile(source)
        val result = value(host.runInCoroutineContext {
            host.evaluator(compiled, ScriptEvaluationConfiguration {
                constructorArgs(emptyArray<String>())
            })
        })
        val returned = result.returnValue
        if (returned is ResultValue.Error) throw returned.error
    }

    private fun <T> value(result: ResultWithDiagnostics<T>): T {
        for (report in result.reports) {
            if (report.severity >= ScriptDiagnostic.Severity.WARNING)
                System.err.println(report.render())
            report.exception?.printStackTrace()
        }
        return when (result) {
            is ResultWithDiagnostics.Success -> result.value
            is ResultWithDiagnostics.Failure -> error("Kotlin script could not be completed")
        }
    }
}
