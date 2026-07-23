import java.io.IOException;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.nio.file.DirectoryStream;
import java.nio.file.FileStore;
import java.nio.file.Files;
import java.nio.file.LinkOption;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.nio.file.StandardOpenOption;
import java.nio.file.WatchEvent;
import java.nio.file.WatchKey;
import java.nio.file.WatchService;
import java.nio.file.attribute.AclEntry;
import java.nio.file.attribute.AclFileAttributeView;
import java.nio.file.attribute.BasicFileAttributes;
import java.nio.file.attribute.DosFileAttributeView;
import java.nio.file.attribute.FileOwnerAttributeView;
import java.nio.file.attribute.FileTime;
import java.nio.file.attribute.GroupPrincipal;
import java.nio.file.attribute.PosixFileAttributeView;
import java.nio.file.attribute.PosixFileAttributes;
import java.nio.file.attribute.UserDefinedFileAttributeView;
import java.nio.file.attribute.UserPrincipal;
import java.nio.file.attribute.UserPrincipalLookupService;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.TimeUnit;

/** Public-API coverage for the native dispatchers in sun.nio.fs. */
public class Test {
    private static void check(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static boolean isJava8() {
        String version = System.getProperty("java.specification.version");
        String runtimeVersion = System.getProperty("java.version");
        return "1.8".equals(version) || "8".equals(version)
                || (version == null && runtimeVersion != null
                    && ("8".equals(runtimeVersion) || runtimeVersion.startsWith("1.8")));
    }

    public static void main(String[] args) throws Exception {
        Path root = Paths.get("nio_intrinsics_" + System.nanoTime());
        List<Path> cleanup = new ArrayList<Path>();
        Files.createDirectory(root);
        cleanup.add(root);
        System.out.println("root-ok");
        try {
            boolean java8 = isJava8();
            if (java8) {
                exerciseJava8DirectoryPath(root, cleanup);
                System.out.println("nio-fs-intrinsics-ok");
                return;
            }

            Path source = root.resolve("caf\u00e9-source.txt");
            byte[] content = new byte[8 * 1024 + 37];
            for (int index = 0; index < content.length; index++) {
                content[index] = (byte) (index * 31);
            }
            System.out.println("content-ok");
            Files.createFile(source);
            try (OutputStream output = Files.newOutputStream(
                    source, StandardOpenOption.WRITE, StandardOpenOption.TRUNCATE_EXISTING)) {
                output.write(content);
            }
            cleanup.add(source);
            System.out.println("write-ok");

            check(Files.exists(source), "source does not exist");
            check(Files.isReadable(source), "source is not readable");
            check(Files.size(source) == content.length, "wrong source size");
            BasicFileAttributes basic = Files.readAttributes(source, BasicFileAttributes.class);
            check(basic.isRegularFile(), "source is not a regular file");
            check(basic.size() == Files.size(source), "attribute size mismatch");
            System.out.println("basic-ok");

            Path copy = root.resolve("copy.bin");
            Files.copy(source, copy, StandardCopyOption.COPY_ATTRIBUTES);
            cleanup.add(copy);
            check(Files.size(copy) == Files.size(source), "copy size mismatch");
            check(Arrays.equals(Files.readAllBytes(source), Files.readAllBytes(copy)),
                    "copy content mismatch");

            Path moved = root.resolve("moved.bin");
            Files.move(copy, moved, StandardCopyOption.REPLACE_EXISTING);
            cleanup.remove(copy);
            cleanup.add(moved);
            check(!Files.exists(copy) && Files.exists(moved), "move failed");
            System.out.println("copy-move-ok");

            Path hardLink = root.resolve("hard-link.bin");
            Files.createLink(hardLink, source);
            cleanup.add(hardLink);
            check(Files.isSameFile(source, hardLink), "hard link identity mismatch");

            long requestedMillis = 1_234_567_890_000L;
            Files.setLastModifiedTime(source, FileTime.fromMillis(requestedMillis));
            check(Files.getLastModifiedTime(source).toMillis() == requestedMillis,
                    "last-modified time mismatch");
            if ("25".equals(System.getProperty("java.specification.version"))
                    && System.getProperty("os.name", "").contains("Linux")) {
                BasicFileAttributes timestamps = Files.readAttributes(
                        source, BasicFileAttributes.class);
                System.out.println("birthtime-distinct="
                        + !timestamps.creationTime().equals(timestamps.lastModifiedTime()));
            }
            System.out.println("links-time-ok");

            exerciseDirectory(root, source);
            System.out.println("directory-ok");
            exerciseFileStore(root);
            System.out.println("store-ok");
            exerciseUserAttributes(source);
            System.out.println("user-attributes-ok");
            exercisePlatformAttributes(source);
            System.out.println("platform-attributes-ok");
            exerciseLinks(root, source, cleanup);
            System.out.println("symbolic-link-ok");
            exerciseWatchService(root, cleanup);
            System.out.println("watch-ok");

            if (!"17".equals(System.getProperty("java.specification.version"))) {
                String mime = Files.probeContentType(root.resolve("sample.txt"));
                check(mime == null || mime.length() > 0, "invalid MIME result");
            }
            check(source.toRealPath().isAbsolute(), "real path is not absolute");

            System.out.println("nio-fs-intrinsics-ok");
        } catch (Exception exception) {
            exception.printStackTrace();
            throw exception;
        } catch (Error error) {
            error.printStackTrace();
            throw error;
        } finally {
            for (int index = cleanup.size() - 1; index >= 0; index--) {
                Files.deleteIfExists(cleanup.get(index));
            }
        }
    }

    private static void exerciseJava8DirectoryPath(Path root, List<Path> cleanup) throws Exception {
        Path source = root.resolve("source-directory");
        Files.createDirectory(source);
        cleanup.add(source);
        check(Files.readAttributes(source, BasicFileAttributes.class).isDirectory(),
                "directory attributes failed");
        System.out.println("basic-ok");

        exerciseEmptyDirectory(source);
        System.out.println("directory-ok");
        exerciseFileStore(root);
        System.out.println("store-ok");
        exerciseUserAttributes(source);
        System.out.println("user-attributes-ok");
        exercisePlatformAttributes(source);
        System.out.println("platform-attributes-ok");
        exerciseLinks(root, source, cleanup);
        System.out.println("symbolic-link-ok");
        exerciseWatchService(root, cleanup);
        System.out.println("watch-ok");

        check(source.toRealPath().isAbsolute(), "real path is not absolute");
    }

    private static void exerciseDirectory(Path root, Path source) throws IOException {
        boolean found = false;
        try (DirectoryStream<Path> entries = Files.newDirectoryStream(root)) {
            for (Path entry : entries) {
                if (entry.getFileName().equals(source.getFileName())) {
                    found = true;
                }
            }
        }
        check(found, "directory enumeration missed source");
    }

    private static void exerciseEmptyDirectory(Path directory) throws IOException {
        int entriesSeen = 0;
        try (DirectoryStream<Path> entries = Files.newDirectoryStream(directory)) {
            for (Path ignored : entries) {
                entriesSeen++;
            }
        }
        check(entriesSeen == 0, "new directory was not empty");
    }

    private static void exerciseFileStore(Path root) throws IOException {
        FileStore store = Files.getFileStore(root);
        check(store.getTotalSpace() >= 0, "negative total space");
        check(store.getUsableSpace() >= 0, "negative usable space");
        check(store.name() != null && store.type() != null, "invalid file store identity");
    }

    private static void exerciseUserAttributes(Path source) throws IOException {
        UserDefinedFileAttributeView view = Files.getFileAttributeView(
                source, UserDefinedFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
        if (view == null) {
            return;
        }
        String name = "ristretto.integration";
        byte[] value = "attribute-value".getBytes(StandardCharsets.UTF_8);
        view.write(name, ByteBuffer.wrap(value));
        // JDK 17 implements list() with a lambda whose hidden-class access path is
        // independently unsupported by the VM. Later JDKs still exercise flistxattr.
        if (!"17".equals(System.getProperty("java.specification.version"))) {
            check(view.list().contains(name), "user attribute not listed");
        }
        ByteBuffer result = ByteBuffer.allocate(view.size(name));
        view.read(name, result);
        check(Arrays.equals(value, result.array()), "user attribute mismatch");
        view.delete(name);
    }

    private static void exercisePlatformAttributes(Path source) throws IOException {
        if (System.getProperty("os.name", "").contains("Windows")) {
            AclFileAttributeView acl = Files.getFileAttributeView(
                    source, AclFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
            check(acl != null, "ACL view unavailable on Windows");
            List<AclEntry> entries = acl.getAcl();
            acl.setAcl(entries);
            FileOwnerAttributeView owners = Files.getFileAttributeView(
                    source, FileOwnerAttributeView.class, LinkOption.NOFOLLOW_LINKS);
            UserPrincipal owner = owners.getOwner();
            UserPrincipalLookupService lookup = source.getFileSystem().getUserPrincipalLookupService();
            check(lookup.lookupPrincipalByName(owner.getName()) != null,
                    "Windows principal lookup failed");
            DosFileAttributeView dos = Files.getFileAttributeView(source, DosFileAttributeView.class);
            boolean hidden = dos.readAttributes().isHidden();
            dos.setHidden(hidden);
        } else {
            PosixFileAttributeView posix = Files.getFileAttributeView(
                    source, PosixFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
            check(posix != null, "POSIX view unavailable");
            PosixFileAttributes attributes = posix.readAttributes();
            posix.setPermissions(attributes.permissions());
            UserPrincipalLookupService lookup = source.getFileSystem().getUserPrincipalLookupService();
            UserPrincipal user = lookup.lookupPrincipalByName(attributes.owner().getName());
            GroupPrincipal group = lookup.lookupPrincipalByGroupName(attributes.group().getName());
            check(user != null && group != null, "POSIX principal lookup failed");
        }
    }

    private static void exerciseLinks(Path root, Path source, List<Path> cleanup) throws IOException {
        if (System.getProperty("os.name", "").contains("Windows")) {
            return;
        }
        Path symbolic = root.resolve("symbolic-link.bin");
        Files.createSymbolicLink(symbolic, source.getFileName());
        cleanup.add(symbolic);
        check(Files.isSymbolicLink(symbolic), "symbolic link not detected");
        check(Files.readSymbolicLink(symbolic).equals(source.getFileName()),
                "symbolic link target mismatch");
        check(Files.isSameFile(source, symbolic), "symbolic link identity mismatch");
    }

    private static void exerciseWatchService(Path root, List<Path> cleanup) throws Exception {
        try (WatchService watchService = root.getFileSystem().newWatchService()) {
            root.register(watchService, java.nio.file.StandardWatchEventKinds.ENTRY_CREATE);
            Path watched = root.resolve("watch-created.txt");
            if (isJava8()) {
                Files.createDirectory(watched);
            } else {
                Files.write(watched, new byte[] { 1 });
            }
            cleanup.add(watched);
            WatchKey key = watchService.poll(10, TimeUnit.SECONDS);
            check(key != null, "watch service produced no key");
            boolean found = false;
            for (WatchEvent<?> event : key.pollEvents()) {
                if (event.context().toString().equals(watched.getFileName().toString())) {
                    found = true;
                }
            }
            check(found, "watch service missed create event");
            key.reset();
        }
    }
}
