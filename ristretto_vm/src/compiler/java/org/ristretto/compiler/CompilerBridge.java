package org.ristretto.compiler;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.Reader;
import java.io.StringReader;
import java.io.Writer;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import javax.tools.FileObject;
import javax.tools.ForwardingJavaFileManager;
import javax.tools.JavaCompiler;
import javax.tools.JavaFileManager;
import javax.tools.JavaFileObject;
import javax.tools.SimpleJavaFileObject;
import javax.tools.StandardJavaFileManager;
import javax.tools.ToolProvider;

/** Internal bridge between Rust and the standard Java compiler API. */
public final class CompilerBridge {
    private CompilerBridge() {
    }

    /** Compile named sources and return a status followed by class-name/byte-array pairs. */
    public static Object[] compile(String[] names, String[] sources, String[] options) {
        if (names == null || sources == null || options == null || names.length != sources.length) {
            return result(2, null);
        }

        JavaCompiler compiler = ToolProvider.getSystemJavaCompiler();
        if (compiler == null) {
            return result(3, null);
        }

        StandardJavaFileManager standard = compiler.getStandardFileManager(null, null, null);
        MemoryFileManager files = new MemoryFileManager(standard);
        try {
            List<JavaFileObject> units = new ArrayList<JavaFileObject>(names.length);
            for (int index = 0; index < names.length; index++) {
                units.add(new Source(names[index], sources[index]));
            }
            JavaCompiler.CompilationTask task = compiler.getTask(
                    null, files, null, Arrays.asList(options), null, units);
            boolean successful = Boolean.TRUE.equals(task.call());
            return result(successful ? 0 : 1, files.classBytes());
        } catch (IllegalArgumentException exception) {
            printMessage(exception);
            return result(2, null);
        } catch (Exception exception) {
            printMessage(exception);
            return result(3, null);
        } catch (Throwable throwable) {
            printMessage(throwable);
            return result(4, null);
        } finally {
            try {
                files.close();
            } catch (IOException exception) {
                printMessage(exception);
            }
        }
    }

    private static void printMessage(Throwable throwable) {
        String message = throwable.getMessage();
        if (message != null) {
            System.err.println(message);
        }
    }

    private static Object[] result(int status, Map<String, byte[]> classes) {
        int classCount = classes == null ? 0 : classes.size();
        Object[] result = new Object[1 + classCount * 2];
        result[0] = Integer.valueOf(status);
        if (classes != null) {
            int index = 1;
            for (Map.Entry<String, byte[]> entry : classes.entrySet()) {
                result[index++] = entry.getKey();
                result[index++] = entry.getValue();
            }
        }
        return result;
    }

    private static final class Source extends SimpleJavaFileObject {
        private final String binaryName;
        private final String source;

        Source(String binaryName, String source) {
            super(uri(binaryName, Kind.SOURCE), Kind.SOURCE);
            this.binaryName = binaryName;
            this.source = source;
        }

        @Override
        public CharSequence getCharContent(boolean ignoreEncodingErrors) {
            return source;
        }
    }

    private static final class MemoryOutput extends SimpleJavaFileObject {
        private final String binaryName;
        private final ByteArrayOutputStream bytes = new ByteArrayOutputStream();

        MemoryOutput(String binaryName, Kind kind) {
            super(uri(binaryName, kind), kind);
            this.binaryName = binaryName;
        }

        @Override
        public OutputStream openOutputStream() {
            bytes.reset();
            return bytes;
        }

        @Override
        public Writer openWriter() {
            bytes.reset();
            return new java.io.OutputStreamWriter(bytes, StandardCharsets.UTF_8);
        }

        @Override
        public InputStream openInputStream() {
            return new ByteArrayInputStream(bytes.toByteArray());
        }

        @Override
        public Reader openReader(boolean ignoreEncodingErrors) {
            return new StringReader(new String(bytes.toByteArray(), StandardCharsets.UTF_8));
        }

        @Override
        public CharSequence getCharContent(boolean ignoreEncodingErrors) {
            return new String(bytes.toByteArray(), StandardCharsets.UTF_8);
        }

        byte[] toByteArray() {
            return bytes.toByteArray();
        }
    }

    private static final class MemoryFileManager
            extends ForwardingJavaFileManager<StandardJavaFileManager> {
        private final Map<String, MemoryOutput> outputs = new HashMap<String, MemoryOutput>();
        private final Map<String, MemoryOutput> classes = new TreeMap<String, MemoryOutput>();

        MemoryFileManager(StandardJavaFileManager fileManager) {
            super(fileManager);
        }

        @Override
        public JavaFileObject getJavaFileForOutput(
                JavaFileManager.Location location,
                String className,
                JavaFileObject.Kind kind,
                FileObject sibling) {
            MemoryOutput output = new MemoryOutput(className, kind);
            outputs.put(key(location, className, kind), output);
            if (kind == JavaFileObject.Kind.CLASS) {
                classes.put(className, output);
            }
            return output;
        }

        @Override
        public FileObject getFileForOutput(
                JavaFileManager.Location location,
                String packageName,
                String relativeName,
                FileObject sibling) {
            String name = packageName.length() == 0
                    ? relativeName
                    : packageName + "." + relativeName;
            MemoryOutput output = new MemoryOutput(name, JavaFileObject.Kind.OTHER);
            outputs.put(key(location, name, JavaFileObject.Kind.OTHER), output);
            return output;
        }

        @Override
        public JavaFileObject getJavaFileForInput(
                JavaFileManager.Location location,
                String className,
                JavaFileObject.Kind kind) throws IOException {
            MemoryOutput output = outputs.get(key(location, className, kind));
            return output == null ? super.getJavaFileForInput(location, className, kind) : output;
        }

        @Override
        public String inferBinaryName(JavaFileManager.Location location, JavaFileObject file) {
            if (file instanceof Source) {
                return ((Source) file).binaryName;
            }
            if (file instanceof MemoryOutput) {
                return ((MemoryOutput) file).binaryName;
            }
            return super.inferBinaryName(location, file);
        }

        Map<String, byte[]> classBytes() {
            Map<String, byte[]> result = new TreeMap<String, byte[]>();
            for (Map.Entry<String, MemoryOutput> entry : classes.entrySet()) {
                result.put(entry.getKey(), entry.getValue().toByteArray());
            }
            return result;
        }

        private static String key(
                JavaFileManager.Location location, String name, JavaFileObject.Kind kind) {
            return location.getName() + ':' + name + ':' + kind.name();
        }
    }

    private static URI uri(String binaryName, JavaFileObject.Kind kind) {
        return URI.create("mem:///" + binaryName.replace('.', '/') + kind.extension);
    }
}
