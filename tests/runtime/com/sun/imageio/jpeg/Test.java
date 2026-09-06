import java.awt.Rectangle;
import java.awt.image.BufferedImage;
import java.awt.image.Raster;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import javax.imageio.IIOException;
import javax.imageio.IIOImage;
import javax.imageio.ImageIO;
import javax.imageio.ImageReadParam;
import javax.imageio.ImageReader;
import javax.imageio.ImageWriteParam;
import javax.imageio.ImageWriter;
import javax.imageio.event.IIOReadProgressListener;
import javax.imageio.event.IIOWriteProgressListener;
import javax.imageio.plugins.jpeg.JPEGImageWriteParam;
import javax.imageio.stream.MemoryCacheImageInputStream;
import javax.imageio.stream.MemoryCacheImageOutputStream;

/** Exercises the real JDK ImageIO providers and their native callbacks. */
public class Test {
    public static void main(String[] args) throws Exception {
        try {
            ImageIO.setUseCache(false);
            testReferenceAndStreams();
            testRoundTrip(false, false);
            testRoundTrip(true, false);
            testRoundTrip(false, true);
            testRegions();
            testAbbreviatedTables();
            testProgressivePasses();
            testProgressiveSamples();
            testCustomTables();
            testMetadataAndOutputFailure();
            testSequence();
            testAbortAndReuse();
            testErrors();
            System.out.println("JPEG ImageIO integration: passed");
        } catch (Throwable error) {
            error.printStackTrace(System.out);
            for (Throwable cause = error; cause != null; cause = cause instanceof ExceptionInInitializerError
                    ? ((ExceptionInInitializerError)cause).getException() : cause.getCause()) {
                System.out.println(cause.getClass().getName() + ": " + cause.getMessage());
            }
            throw error;
        }
    }
    // Baseline grayscale fixture encoded by OpenJDK, independent of Ristretto's writer.
    private static final String REFERENCE = "/9j/4AAQSkZJRgABAgAAAQABAAD/2wBDAAgGBgcGBQgHBwcJCQgKDBQNDAsLDBkSEw8UHRofHh0aHBwgJC4nICIsIxwcKDcpLDAxNDQ0Hyc5PTgyPC4zNDL/wAALCAAQABABAREA/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/9oACAEBAAA/APN9H0f7vy16Bo+j/d+WjR9H+78tegaPo/3flr//2Q==";

    private static void testReferenceAndStreams() throws Exception {
        byte[] jpeg = java.util.Base64.getDecoder().decode(REFERENCE);
        byte[] withTrailer = java.util.Arrays.copyOf(jpeg, jpeg.length + 3);
        withTrailer[jpeg.length] = 42;
        ImageReader reader = reader();
        ByteArrayInputStream shortReads = new ByteArrayInputStream(withTrailer) {
            @Override public synchronized int read(byte[] bytes, int offset, int length) {
                return super.read(bytes, offset, Math.min(length, 3));
            }
        };
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(shortReads)) {
            reader.setInput(in);
            Raster raster = reader.readRaster(0, null);
            for (int y = 0; y < 16; y++) for (int x = 0; x < 16; x++) {
                check(Math.abs(raster.getSample(x, y, 0) - (x + y) * 8) <= 3, "reference pixels");
            }
            check(in.getStreamPosition() == jpeg.length, "reader must push back trailing bytes");
            check(in.read() == 42, "trailing byte preserved");
        } finally { reader.dispose(); }
        // Querying dimensions requires only the header, without consuming scan data.
        int scan = 0;
        for (int i = 0; i + 1 < jpeg.length; i++) {
            if ((jpeg[i] & 255) == 255 && (jpeg[i + 1] & 255) == 218) { scan = i; break; }
        }
        int end = scan + 2 + ((jpeg[scan + 2] & 255) << 8) + (jpeg[scan + 3] & 255);
        reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(
                new ByteArrayInputStream(java.util.Arrays.copyOf(jpeg, end)))) {
            reader.setInput(in, false, true);
            check(reader.getWidth(0) == 16 && reader.getHeight(0) == 16, "header-only dimensions");
        } finally { reader.dispose(); }
        reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(jpeg)) {
            @Override public int read(byte[] data, int offset, int length) throws IOException {
                throw new IOException("input sentinel");
            }
        }) {
            reader.setInput(in);
            try { reader.readRaster(0, null); throw new AssertionError("input failure lost"); }
            catch (IOException expected) { check("input sentinel".equals(expected.getMessage()), "input exception identity"); }
        } finally { reader.dispose(); }
    }

    private static void check(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }
    private static BufferedImage image(boolean gray) {
        BufferedImage image = new BufferedImage(24, 16,
                gray ? BufferedImage.TYPE_BYTE_GRAY : BufferedImage.TYPE_INT_RGB);
        for (int y = 0; y < image.getHeight(); y++) {
            for (int x = 0; x < image.getWidth(); x++) {
                if (gray) image.getRaster().setSample(x, y, 0, 128);
                else image.setRGB(x, y, 0xc03020);
            }
        }
        return image;
    }
    private static ImageWriter writer() { return ImageIO.getImageWritersByFormatName("JPEG").next(); }
    private static ImageReader reader() { return ImageIO.getImageReadersByFormatName("JPEG").next(); }
    private static byte[] encode(BufferedImage image, boolean progressive) throws Exception {
        ImageWriter writer = writer();
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(bytes)) {
            writer.setOutput(out);
            JPEGImageWriteParam param = (JPEGImageWriteParam)writer.getDefaultWriteParam();
            param.setCompressionMode(ImageWriteParam.MODE_EXPLICIT);
            param.setCompressionQuality(0.9f);
            param.setOptimizeHuffmanTables(true);
            param.setProgressiveMode(progressive ? ImageWriteParam.MODE_DEFAULT : ImageWriteParam.MODE_DISABLED);
            writer.write(null, new IIOImage(image, null, null), param);
        } finally { writer.dispose(); }
        return bytes.toByteArray();
    }
    private static void testRoundTrip(boolean progressive, boolean gray) throws Exception {
        byte[] jpeg = encode(image(gray), progressive);
        check((jpeg[0] & 255) == 255 && (jpeg[1] & 255) == 216, "SOI");
        check((jpeg[jpeg.length - 2] & 255) == 255 && (jpeg[jpeg.length - 1] & 255) == 217, "EOI");
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(jpeg))) {
            reader.setInput(in);
            check(reader.getWidth(0) == 24 && reader.getHeight(0) == 16, "header dimensions");
            check(reader.getImageMetadata(0) != null, "image metadata");
            BufferedImage decoded = reader.read(0);
            check(decoded.getWidth() == 24 && decoded.getHeight() == 16, "decoded dimensions");
            if (gray) check(Math.abs(decoded.getRaster().getSample(12, 8, 0) - 128) <= 1, "gray sample");
            int rgb = gray ? 0x808080 : decoded.getRGB(12, 8);
            int expected = gray ? 0x808080 : 0xc03020;
            for (int shift = 0; shift <= 16; shift += 8) {
                check(Math.abs(((rgb >> shift) & 255) - ((expected >> shift) & 255)) <= 5, "decoded pixel");
            }
            Raster raster = reader.readRaster(0, null);
            check(raster.getNumBands() == (gray ? 1 : 3), "raw raster components");
            reader.reset();
        } finally { reader.dispose(); }
    }
    private static void testRegions() throws Exception {
        byte[] jpeg = encode(image(false), false);
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(jpeg))) {
            reader.setInput(in);
            ImageReadParam param = reader.getDefaultReadParam();
            param.setSourceRegion(new Rectangle(2, 4, 16, 8));
            param.setSourceSubsampling(2, 2, 0, 0);
            BufferedImage decoded = reader.read(0, param);
            check(decoded.getWidth() == 8 && decoded.getHeight() == 4, "read region and subsampling");
        } finally { reader.dispose(); }
        ImageWriter writer = writer();
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(bytes)) {
            writer.setOutput(out);
            ImageWriteParam param = writer.getDefaultWriteParam();
            param.setSourceRegion(new Rectangle(2, 4, 16, 8));
            param.setSourceSubsampling(2, 2, 0, 0);
            writer.write(null, new IIOImage(image(false), null, null), param);
        } finally { writer.dispose(); }
        BufferedImage decoded = ImageIO.read(new ByteArrayInputStream(bytes.toByteArray()));
        check(decoded.getWidth() == 8 && decoded.getHeight() == 4, "write region and subsampling");
    }
    private static void testAbbreviatedTables() throws Exception {
        byte[] jpeg = java.util.Base64.getDecoder().decode(REFERENCE);
        ByteArrayOutputStream stripped = new ByteArrayOutputStream();
        stripped.write(jpeg, 0, 2);
        int pos = 2;
        while (pos < jpeg.length) {
            int marker = jpeg[pos + 1] & 255;
            if (marker == 218) { stripped.write(jpeg, pos, jpeg.length - pos); break; }
            int length = ((jpeg[pos + 2] & 255) << 8) + (jpeg[pos + 3] & 255) + 2;
            if (marker != 219 && marker != 196) stripped.write(jpeg, pos, length);
            pos += length;
        }
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(stripped.toByteArray()))) {
            reader.setInput(in);
            javax.imageio.plugins.jpeg.JPEGImageReadParam param =
                    (javax.imageio.plugins.jpeg.JPEGImageReadParam)reader.getDefaultReadParam();
            param.setDecodeTables(
                new javax.imageio.plugins.jpeg.JPEGQTable[] { javax.imageio.plugins.jpeg.JPEGQTable.K1Div2Luminance },
                new javax.imageio.plugins.jpeg.JPEGHuffmanTable[] { javax.imageio.plugins.jpeg.JPEGHuffmanTable.StdDCLuminance },
                new javax.imageio.plugins.jpeg.JPEGHuffmanTable[] { javax.imageio.plugins.jpeg.JPEGHuffmanTable.StdACLuminance });
            Raster raster = reader.readRaster(0, param);
            check(Math.abs(raster.getSample(12, 12, 0) - 192) <= 3, "abbreviated decoding tables");
        } finally { reader.dispose(); }
    }
    private static void testProgressivePasses() throws Exception {
        byte[] jpeg = encode(image(false), true);
        ImageReader reader = reader();
        final int[] passes = { 0, 0 };
        reader.addIIOReadUpdateListener(new javax.imageio.event.IIOReadUpdateListener() {
            public void passStarted(ImageReader source, BufferedImage image, int pass, int min, int max,
                    int minX, int minY, int periodX, int periodY, int[] bands) {
                check(pass >= 1 && pass <= 2, "progressive pass range"); passes[0]++;
            }
            public void passComplete(ImageReader source, BufferedImage image) { passes[1]++; }
            public void imageUpdate(ImageReader source, BufferedImage image, int minX, int minY,
                    int width, int height, int periodX, int periodY, int[] bands) {}
            public void thumbnailPassStarted(ImageReader source, BufferedImage image, int pass, int min, int max,
                    int minX, int minY, int periodX, int periodY, int[] bands) {}
            public void thumbnailPassComplete(ImageReader source, BufferedImage image) {}
            public void thumbnailUpdate(ImageReader source, BufferedImage image, int minX, int minY,
                    int width, int height, int periodX, int periodY, int[] bands) {}
        });
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(jpeg))) {
            reader.setInput(in);
            ImageReadParam param = reader.getDefaultReadParam();
            param.setSourceProgressivePasses(1, 2);
            reader.read(0, param);
            check(passes[0] == 2 && passes[1] == 2, "progressive pass callbacks");
        } finally { reader.dispose(); }
    }
    private static BufferedImage detailedGray() {
        BufferedImage source = new BufferedImage(31, 19, BufferedImage.TYPE_BYTE_GRAY);
        for (int y = 0; y < 19; y++) for (int x = 0; x < 31; x++) {
            source.getRaster().setSample(x, y, 0, (x * 13 + y * 19 + x * y % 47) & 255);
        }
        return source;
    }
    private static void testProgressiveSamples() throws Exception {
        BufferedImage source = detailedGray();
        byte[] baseline = encode(source, false);
        byte[] progressive = encode(source, true);
        Raster sequential = ImageIO.read(new ByteArrayInputStream(baseline)).getRaster();
        Raster complete = ImageIO.read(new ByteArrayInputStream(progressive)).getRaster();
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(progressive))) {
            reader.setInput(in);
            ImageReadParam param = reader.getDefaultReadParam();
            param.setSourceProgressivePasses(0, 1);
            Raster first = reader.read(0, param).getRaster();
            int changed = 0;
            for (int y = 0; y < 19; y++) for (int x = 0; x < 31; x++) {
                int value = complete.getSample(x, y, 0);
                check(Math.abs(value - source.getRaster().getSample(x, y, 0)) <= 25, "progressive AC sample");
                check(Math.abs(value - sequential.getSample(x, y, 0)) <= 1, "successive approximation final samples");
                if (Math.abs(value - first.getSample(x, y, 0)) > 10) changed++;
            }
            check(changed > 100, "early progressive pass must contain an intermediate image");
        } finally { reader.dispose(); }
    }
    private static void testCustomTables() throws Exception {
        BufferedImage source = detailedGray();
        ImageWriter writer = writer();
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        int[] quant = new int[64];
        java.util.Arrays.fill(quant, 19);
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(bytes)) {
            writer.setOutput(out);
            JPEGImageWriteParam param = (JPEGImageWriteParam)writer.getDefaultWriteParam();
            param.setOptimizeHuffmanTables(false);
            javax.imageio.metadata.IIOMetadata metadata = writer.getDefaultImageMetadata(
                    javax.imageio.ImageTypeSpecifier.createFromRenderedImage(source), param);
            javax.imageio.metadata.IIOMetadataNode root = (javax.imageio.metadata.IIOMetadataNode)
                    metadata.getAsTree("javax_imageio_jpeg_image_1.0");
            javax.imageio.metadata.IIOMetadataNode dqt = (javax.imageio.metadata.IIOMetadataNode)
                    root.getElementsByTagName("dqt").item(0);
            while (dqt.getFirstChild() != null) dqt.removeChild(dqt.getFirstChild());
            for (int i = 0; i < 4; i++) {
                javax.imageio.metadata.IIOMetadataNode table = new javax.imageio.metadata.IIOMetadataNode("dqtable");
                table.setAttribute("qtableId", Integer.toString(i));
                int[] values = quant.clone();
                if (i != 3) java.util.Arrays.fill(values, 3 + i);
                table.setUserObject(new javax.imageio.plugins.jpeg.JPEGQTable(values));
                dqt.appendChild(table);
            }
            ((javax.imageio.metadata.IIOMetadataNode)root.getElementsByTagName("componentSpec").item(0))
                    .setAttribute("QtableSelector", "3");
            org.w3c.dom.NodeList huffman = root.getElementsByTagName("dhtable");
            for (int i = 0; i < huffman.getLength(); i++) {
                javax.imageio.metadata.IIOMetadataNode node = (javax.imageio.metadata.IIOMetadataNode)huffman.item(i);
                javax.imageio.plugins.jpeg.JPEGHuffmanTable original =
                        (javax.imageio.plugins.jpeg.JPEGHuffmanTable)node.getUserObject();
                short[] values = original.getValues();
                short swap = values[0]; values[0] = values[1]; values[1] = swap;
                node.setUserObject(new javax.imageio.plugins.jpeg.JPEGHuffmanTable(original.getLengths(), values));
            }
            metadata.setFromTree("javax_imageio_jpeg_image_1.0", root);
            writer.write(null, new IIOImage(source, null, metadata), param);
        } finally { writer.dispose(); }
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(bytes.toByteArray()))) {
            reader.setInput(in);
            javax.imageio.metadata.IIOMetadataNode root = (javax.imageio.metadata.IIOMetadataNode)
                    reader.getImageMetadata(0).getAsTree("javax_imageio_jpeg_image_1.0");
            check("3".equals(((javax.imageio.metadata.IIOMetadataNode)root.getElementsByTagName("componentSpec").item(0))
                    .getAttribute("QtableSelector")), "custom quantization selector");
            Raster decoded = reader.readRaster(0, null);
            for (int y = 0; y < 19; y++) for (int x = 0; x < 31; x++) {
                check(Math.abs(decoded.getSample(x, y, 0) - source.getRaster().getSample(x, y, 0)) <= 25,
                        "custom quantization and Huffman samples");
            }
        } finally { reader.dispose(); }
    }
    private static void testMetadataAndOutputFailure() throws Exception {
        ImageWriter writer = writer();
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        BufferedImage image = image(false);
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(bytes)) {
            writer.setOutput(out);
            ImageWriteParam param = writer.getDefaultWriteParam();
            javax.imageio.metadata.IIOMetadata metadata = writer.getDefaultImageMetadata(
                    javax.imageio.ImageTypeSpecifier.createFromRenderedImage(image), param);
            String format = "javax_imageio_jpeg_image_1.0";
            javax.imageio.metadata.IIOMetadataNode root =
                    (javax.imageio.metadata.IIOMetadataNode)metadata.getAsTree(format);
            javax.imageio.metadata.IIOMetadataNode sequence =
                    (javax.imageio.metadata.IIOMetadataNode)root.getElementsByTagName("markerSequence").item(0);
            javax.imageio.metadata.IIOMetadataNode comment = new javax.imageio.metadata.IIOMetadataNode("com");
            comment.setAttribute("comment", "ImageIO metadata callback"); sequence.appendChild(comment);
            javax.imageio.metadata.IIOMetadataNode restart = new javax.imageio.metadata.IIOMetadataNode("dri");
            restart.setAttribute("interval", "1"); sequence.appendChild(restart);
            metadata.setFromTree(format, root);
            writer.write(null, new IIOImage(image, null, metadata), param);
        } finally { writer.dispose(); }
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(bytes.toByteArray()))) {
            reader.setInput(in);
            javax.imageio.metadata.IIOMetadataNode root = (javax.imageio.metadata.IIOMetadataNode)
                    reader.getImageMetadata(0).getAsTree("javax_imageio_jpeg_image_1.0");
            check("ImageIO metadata callback".equals(((javax.imageio.metadata.IIOMetadataNode)
                    root.getElementsByTagName("com").item(0)).getAttribute("comment")), "comment metadata");
            check("1".equals(((javax.imageio.metadata.IIOMetadataNode)
                    root.getElementsByTagName("dri").item(0)).getAttribute("interval")), "restart interval");
            check(reader.read(0).getWidth() == 24, "restart marker decoding");
        } finally { reader.dispose(); }
        writer = writer();
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(new ByteArrayOutputStream()) {
            @Override public void write(byte[] bytes, int offset, int length) throws IOException {
                throw new IOException("output sentinel");
            }
        }) {
            writer.setOutput(out);
            try { writer.write(image); throw new AssertionError("output failure lost"); }
            catch (IOException expected) { check("output sentinel".equals(expected.getMessage()), "output exception identity"); }
            writer.reset();
        } finally { writer.dispose(); }
    }
    private static void testSequence() throws Exception {
        ImageWriter writer = writer();
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(bytes)) {
            writer.setOutput(out);
            writer.prepareWriteSequence(writer.getDefaultStreamMetadata(null));
            writer.writeToSequence(new IIOImage(image(false), null, null), null);
            writer.writeToSequence(new IIOImage(image(false), null, null), null);
            writer.endWriteSequence();
        } finally { writer.dispose(); }
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(bytes.toByteArray()))) {
            reader.setInput(in);
            check(reader.getNumImages(true) == 2, "sequence image count");
            check(reader.getStreamMetadata() != null, "tables-only stream metadata");
            check(reader.read(1).getWidth() == 24, "second sequence image");
            check(reader.read(0).getHeight() == 16, "seek back to first image");
        } finally { reader.dispose(); }
    }
    private static void testAbortAndReuse() throws Exception {
        ImageWriter writer = writer();
        final boolean[] writeAborted = { false };
        writer.addIIOWriteProgressListener(new IIOWriteProgressListener() {
            public void imageStarted(ImageWriter source, int index) { source.abort(); }
            public void imageProgress(ImageWriter source, float percentage) {}
            public void imageComplete(ImageWriter source) {}
            public void thumbnailStarted(ImageWriter source, int image, int thumbnail) {}
            public void thumbnailProgress(ImageWriter source, float percentage) {}
            public void thumbnailComplete(ImageWriter source) {}
            public void writeAborted(ImageWriter source) { writeAborted[0] = true; }
        });
        try (MemoryCacheImageOutputStream out = new MemoryCacheImageOutputStream(new ByteArrayOutputStream())) {
            writer.setOutput(out);
            writer.write(image(false));
            check(writeAborted[0], "writer abort callback");
            writer.reset();
        } finally { writer.dispose(); }
        byte[] jpeg = encode(image(false), false);
        ImageReader reader = reader();
        final boolean[] readAborted = { false };
        reader.addIIOReadProgressListener(new IIOReadProgressListener() {
            public void imageStarted(ImageReader source, int index) { source.abort(); }
            public void imageProgress(ImageReader source, float percentage) {}
            public void imageComplete(ImageReader source) {}
            public void sequenceStarted(ImageReader source, int minimum) {}
            public void sequenceComplete(ImageReader source) {}
            public void thumbnailStarted(ImageReader source, int image, int thumbnail) {}
            public void thumbnailProgress(ImageReader source, float percentage) {}
            public void thumbnailComplete(ImageReader source) {}
            public void readAborted(ImageReader source) { readAborted[0] = true; }
        });
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(jpeg))) {
            reader.setInput(in);
            reader.read(0);
            check(readAborted[0], "reader abort callback");
            reader.removeAllIIOReadProgressListeners();
            check(reader.read(0).getWidth() == 24, "reader reuse after abort");
        } finally { reader.dispose(); }
    }
    private static void testErrors() throws Exception {
        ImageReader reader = reader();
        try (MemoryCacheImageInputStream in = new MemoryCacheImageInputStream(new ByteArrayInputStream(new byte[] {1, 2, 3, 4}))) {
            reader.setInput(in);
            try { reader.getWidth(0); throw new AssertionError("bad JPEG accepted"); }
            catch (IIOException expected) {}
        } finally { reader.dispose(); }
        reader = reader();
        reader.dispose();
        try { reader.setInput(null); throw new AssertionError("disposed reader accepted input"); }
        catch (IllegalStateException expected) {}
        ImageWriter writer = writer();
        writer.dispose();
        try { writer.setOutput(null); throw new AssertionError("disposed writer accepted output"); }
        catch (IllegalStateException expected) {}
    }
}
