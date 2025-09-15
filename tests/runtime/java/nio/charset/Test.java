import java.nio.*;
import java.nio.charset.*;
import java.util.Arrays;
import java.util.Set;

/**
 * Tests for java.nio.charset
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Java NIO Charset Tests ===");

        testCharsetBasics();
        testCharsetEncoding();
        testCharsetDecoding();
        testCharsetEncoderDecoder();
        testStandardCharsets();
        testCharsetErrors();
        testCoderResult();

        System.out.println("=== Charset Tests Complete ===");
    }

    private static void testCharsetBasics() {
        System.out.println("--- Charset Basics Tests ---");

        // Test default charset
        Charset defaultCharset = Charset.defaultCharset();
        System.out.println("Default charset: " + defaultCharset.name());

        // Test UTF-8 charset
        Charset utf8 = Charset.forName("UTF-8");
        System.out.println("UTF-8 charset name: " + utf8.name());

        // Test charset availability
        System.out.println("UTF-8 is supported: " + Charset.isSupported("UTF-8"));
        System.out.println("ASCII is supported: " + Charset.isSupported("US-ASCII"));
        System.out.println("ISO-8859-1 is supported: " + Charset.isSupported("ISO-8859-1"));
        System.out.println("Invalid charset supported: " + Charset.isSupported("INVALID-CHARSET"));

        // Test invalid charset
        try {
            Charset.forName("INVALID-CHARSET");
            System.out.println("ERROR: Invalid charset should throw exception");
        } catch (UnsupportedCharsetException e) {
            System.out.println("Invalid charset correctly threw: " + e.getClass().getSimpleName());
        }

        // Test available charsets
        Set<String> available = Charset.availableCharsets().keySet();
        System.out.println("Available charsets count: " + available.size());
        System.out.println("UTF-8 in available charsets: " + available.contains("UTF-8"));

        // Test charset aliases
        Set<String> aliases = utf8.aliases();
        System.out.println("UTF-8 aliases count: " + aliases.size());

        // Test charset display name
        String displayName = utf8.displayName();
        System.out.println("UTF-8 display name: " + displayName);

        // Test charset properties
        System.out.println("UTF-8 can encode: " + utf8.canEncode());
        System.out.println("UTF-8 is registered: " + utf8.isRegistered());
    }

    private static void testCharsetEncoding() {
        System.out.println("--- Charset Encoding Tests ---");

        Charset utf8 = Charset.forName("UTF-8");
        String testString = "Hello, 世界! 🌍";

        // Test direct encoding
        ByteBuffer encoded = utf8.encode(testString);
        System.out.println("Original string: '" + testString + "'");
        System.out.println("Encoded buffer remaining: " + encoded.remaining());

        // Test CharBuffer encoding
        CharBuffer charBuffer = CharBuffer.wrap(testString);
        ByteBuffer encoded2 = utf8.encode(charBuffer);
        System.out.println("CharBuffer encoding remaining: " + encoded2.remaining());
        System.out.println("Both encodings same: " + Arrays.equals(encoded.array(), encoded2.array()));

        // Test ASCII encoding
        Charset ascii = Charset.forName("US-ASCII");
        String asciiString = "Hello ASCII";
        ByteBuffer asciiEncoded = ascii.encode(asciiString);
        System.out.println("ASCII string: '" + asciiString + "'");
        System.out.println("ASCII encoded length: " + asciiEncoded.remaining());
        System.out.println("ASCII expected length: " + asciiString.length());

        // Test ISO-8859-1 encoding
        Charset iso88591 = Charset.forName("ISO-8859-1");
        String isoString = "Hello ISO";
        ByteBuffer isoEncoded = iso88591.encode(isoString);
        System.out.println("ISO string: '" + isoString + "'");
        System.out.println("ISO encoded length: " + isoEncoded.remaining());
    }

    private static void testCharsetDecoding() {
        System.out.println("--- Charset Decoding Tests ---");

        Charset utf8 = Charset.forName("UTF-8");
        String original = "Hello, 世界! 🌍";

        // Encode then decode
        ByteBuffer encoded = utf8.encode(original);
        CharBuffer decoded = utf8.decode(encoded);
        System.out.println("Original: '" + original + "'");
        System.out.println("Decoded: '" + decoded.toString() + "'");
        System.out.println("Round-trip successful: " + original.equals(decoded.toString()));

        // Test ASCII round-trip
        Charset ascii = Charset.forName("US-ASCII");
        String asciiString = "Hello ASCII World";
        ByteBuffer asciiEncoded = ascii.encode(asciiString);
        CharBuffer asciiDecoded = ascii.decode(asciiEncoded);
        System.out.println("ASCII original: '" + asciiString + "'");
        System.out.println("ASCII decoded: '" + asciiDecoded.toString() + "'");
        System.out.println("ASCII round-trip: " + asciiString.equals(asciiDecoded.toString()));

        // Test byte array decoding
        byte[] utf8Bytes = original.getBytes(utf8);
        CharBuffer decodedFromBytes = utf8.decode(ByteBuffer.wrap(utf8Bytes));
        System.out.println("Byte array decode: '" + decodedFromBytes.toString() + "'");
        System.out.println("Byte array decode matches: " + original.equals(decodedFromBytes.toString()));
    }

    private static void testCharsetEncoderDecoder() {
        System.out.println("--- CharsetEncoder/Decoder Tests ---");

        Charset utf8 = Charset.forName("UTF-8");
        CharsetEncoder encoder = utf8.newEncoder();
        CharsetDecoder decoder = utf8.newDecoder();

        System.out.println("Encoder charset: " + encoder.charset().name());
        System.out.println("Decoder charset: " + decoder.charset().name());

        // Test encoder properties
        System.out.println("Encoder average bytes per char: " + encoder.averageBytesPerChar());
        System.out.println("Encoder max bytes per char: " + encoder.maxBytesPerChar());

        // Test decoder properties
        System.out.println("Decoder average chars per byte: " + decoder.averageCharsPerByte());
        System.out.println("Decoder max chars per byte: " + decoder.maxCharsPerByte());

        // Test encoding with encoder
        String testString = "Encoder test 测试";
        CharBuffer input = CharBuffer.wrap(testString);
        ByteBuffer output = ByteBuffer.allocate(100);

        CoderResult encodeResult = encoder.encode(input, output, true);
        System.out.println("Encode result: " + encodeResult);
        System.out.println("Encode is underflow: " + encodeResult.isUnderflow());

        encoder.flush(output);
        output.flip();

        // Test decoding with decoder
        CharBuffer decodedOutput = CharBuffer.allocate(100);
        CoderResult decodeResult = decoder.decode(output, decodedOutput, true);
        System.out.println("Decode result: " + decodeResult);
        System.out.println("Decode is underflow: " + decodeResult.isUnderflow());

        decoder.flush(decodedOutput);
        decodedOutput.flip();

        System.out.println("Original: '" + testString + "'");
        System.out.println("Encoded/Decoded: '" + decodedOutput.toString() + "'");
        System.out.println("Encoder/Decoder round-trip: " + testString.equals(decodedOutput.toString()));

        // Test reset
        encoder.reset();
        decoder.reset();
        System.out.println("Encoder and decoder reset completed");
    }

    private static void testStandardCharsets() {
        System.out.println("--- Standard Charsets Tests ---");

        // Test that standard charsets are available
        System.out.println("UTF-8 standard charset: " + StandardCharsets.UTF_8.name());
        System.out.println("UTF-16 standard charset: " + StandardCharsets.UTF_16.name());
        System.out.println("UTF-16BE standard charset: " + StandardCharsets.UTF_16BE.name());
        System.out.println("UTF-16LE standard charset: " + StandardCharsets.UTF_16LE.name());
        System.out.println("US-ASCII standard charset: " + StandardCharsets.US_ASCII.name());
        System.out.println("ISO-8859-1 standard charset: " + StandardCharsets.ISO_8859_1.name());

        // Test encoding with standard charsets
        String testText = "Standard charset test";
        System.out.println("Test text: '" + testText + "'");

        byte[] utf8Bytes = testText.getBytes(StandardCharsets.UTF_8);
        byte[] asciiBytes = testText.getBytes(StandardCharsets.US_ASCII);
        byte[] isoBytes = testText.getBytes(StandardCharsets.ISO_8859_1);

        System.out.println("UTF-8 encoding length: " + utf8Bytes.length);
        System.out.println("ASCII encoding length: " + asciiBytes.length);
        System.out.println("ISO encoding length: " + isoBytes.length);

        // For ASCII text, all should be the same
        boolean utf8AsciiSame = Arrays.equals(utf8Bytes, asciiBytes);
        boolean asciiIsoSame = Arrays.equals(asciiBytes, isoBytes);
        System.out.println("UTF-8 and ASCII same: " + utf8AsciiSame);
        System.out.println("ASCII and ISO same: " + asciiIsoSame);
        System.out.println("All three same for ASCII text: " + (utf8AsciiSame && asciiIsoSame));
    }

    private static void testCharsetErrors() {
        System.out.println("--- Charset Error Handling Tests ---");

        Charset utf8 = Charset.forName("UTF-8");
        Charset ascii = Charset.forName("US-ASCII");

        // Test malformed input handling
        CharsetDecoder decoder = utf8.newDecoder();
        decoder.onMalformedInput(CodingErrorAction.REPORT);

        // Create malformed UTF-8 sequence
        byte[] malformed = {(byte) 0xFF, (byte) 0xFE, (byte) 0xFD};
        ByteBuffer malformedBuffer = ByteBuffer.wrap(malformed);
        CharBuffer output = CharBuffer.allocate(10);

        CoderResult result = decoder.decode(malformedBuffer, output, true);
        System.out.println("Malformed input result: " + result);
        System.out.println("Malformed input detected: " + result.isMalformed());

        // Test unmappable character handling
        CharsetEncoder asciiEncoder = ascii.newEncoder();
        asciiEncoder.onUnmappableCharacter(CodingErrorAction.REPORT);

        String unicodeText = "Hello 世界";
        CharBuffer unicodeInput = CharBuffer.wrap(unicodeText);
        ByteBuffer asciiOutput = ByteBuffer.allocate(20);

        CoderResult encodeResult = asciiEncoder.encode(unicodeInput, asciiOutput, true);
        System.out.println("Unicode to ASCII result: " + encodeResult);
        System.out.println("Unmappable character detected: " + encodeResult.isUnmappable());

        // Test error action replacement
        CharsetEncoder replacingEncoder = ascii.newEncoder();
        replacingEncoder.onUnmappableCharacter(CodingErrorAction.REPLACE);

        unicodeInput.rewind();
        asciiOutput.clear();

        CoderResult replaceResult = replacingEncoder.encode(unicodeInput, asciiOutput, true);
        System.out.println("Replacement encoding result: " + replaceResult);
        System.out.println("Replacement successful: " + replaceResult.isUnderflow());

        // Test replacement string
        byte[] defaultReplacement = replacingEncoder.replacement();
        System.out.println("Default replacement byte: " + (int)defaultReplacement[0]);

        // Test custom replacement
        replacingEncoder.replaceWith(new byte[]{42}); // '*'
        byte[] customReplacement = replacingEncoder.replacement();
        System.out.println("Custom replacement byte: " + (int)customReplacement[0]);
    }

    private static void testCoderResult() {
        System.out.println("--- CoderResult Tests ---");

        // Test underflow result
        CoderResult underflow = CoderResult.UNDERFLOW;
        System.out.println("Underflow is underflow: " + underflow.isUnderflow());
        System.out.println("Underflow is overflow: " + underflow.isOverflow());
        System.out.println("Underflow is error: " + underflow.isError());
        System.out.println("Underflow is malformed: " + underflow.isMalformed());
        System.out.println("Underflow is unmappable: " + underflow.isUnmappable());

        // Test overflow result
        CoderResult overflow = CoderResult.OVERFLOW;
        System.out.println("Overflow is overflow: " + overflow.isOverflow());
        System.out.println("Overflow is underflow: " + overflow.isUnderflow());
        System.out.println("Overflow is error: " + overflow.isError());

        // Test malformed result
        CoderResult malformed = CoderResult.malformedForLength(2);
        System.out.println("Malformed is error: " + malformed.isError());
        System.out.println("Malformed is malformed: " + malformed.isMalformed());
        System.out.println("Malformed is unmappable: " + malformed.isUnmappable());
        System.out.println("Malformed length: " + malformed.length());

        // Test unmappable result
        CoderResult unmappable = CoderResult.unmappableForLength(3);
        System.out.println("Unmappable is error: " + unmappable.isError());
        System.out.println("Unmappable is unmappable: " + unmappable.isUnmappable());
        System.out.println("Unmappable is malformed: " + unmappable.isMalformed());
        System.out.println("Unmappable length: " + unmappable.length());

        // Test toString
        System.out.println("Underflow toString: " + underflow.toString());
        System.out.println("Overflow toString: " + overflow.toString());
        System.out.println("Malformed toString: " + malformed.toString());
        System.out.println("Unmappable toString: " + unmappable.toString());
    }
}
