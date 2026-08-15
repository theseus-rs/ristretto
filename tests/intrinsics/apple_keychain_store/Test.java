import java.util.Base64;
import java.util.concurrent.Callable;

/** Integration coverage for the emulated apple.security.KeychainStore native methods. */
public class Test {
    private static final String EXPECTED =
            "handles positive and distinct: true\n"
            + "key data preserved: true\n"
            + "release hides key data: true\n"
            + "first removal succeeds: true\n"
            + "second removal reports missing: true\n"
            + "second item removal succeeds: true\n"
            + "certificate data unavailable: true\n"
            + "certificate removal succeeds: true\n"
            + "null data becomes empty: true\n"
            + "null item removal succeeds: true\n"
            + "scan succeeds: true\n";

    // Java 8 class file generated from bridge/apple/security/KeychainStore.java.
    private static final String BRIDGE_BYTES =
            "yv66vgAAADQAeQoAAgADBwAEDAAFAAYBABBqYXZhL2xhbmcvT2JqZWN0AQAGPGluaXQ+AQADKClWCAAIAQAFZmlyc3QKAAoACwcA"
            + "DAwADQAOAQAcYXBwbGUvc2VjdXJpdHkvS2V5Y2hhaW5TdG9yZQEAEl9hZGRJdGVtVG9LZXljaGFpbgEAGihMamF2YS9sYW5nL1N0"
            + "cmluZztaW0JbQylKCAAQAQAGc2Vjb25kCgAKABIMABMAFAEAEl9nZXRFbmNvZGVkS2V5RGF0YQEAByhKW0MpW0IKABYAFwcAGAwA"
            + "GQAaAQAQamF2YS91dGlsL0FycmF5cwEABmVxdWFscwEAByhbQltCKVoKAAoAHAwAHQAeAQAXX3JlbGVhc2VLZXljaGFpbkl0ZW1S"
            + "ZWYBAAQoSilWCgAKACAMACEAIgEAF19yZW1vdmVJdGVtRnJvbUtleWNoYWluAQAEKEopSQgAJAEAC2NlcnRpZmljYXRlCAAmAQAa"
            + "amF2YS5zcGVjaWZpY2F0aW9uLnZlcnNpb24KACgAKQcAKgwAKwAsAQAQamF2YS9sYW5nL1N5c3RlbQEAC2dldFByb3BlcnR5AQAm"
            + "KExqYXZhL2xhbmcvU3RyaW5nOylMamF2YS9sYW5nL1N0cmluZzsIAC4BAAIxLggAMAEAAAoAMgAzBwA0DAA1ADYBABBqYXZhL2xh"
            + "bmcvU3RyaW5nAQAHcmVwbGFjZQEARChMamF2YS9sYW5nL0NoYXJTZXF1ZW5jZTtMamF2YS9sYW5nL0NoYXJTZXF1ZW5jZTspTGph"
            + "dmEvbGFuZy9TdHJpbmc7CgA4ADkHADoMADsAPAEAEWphdmEvbGFuZy9JbnRlZ2VyAQAIcGFyc2VJbnQBABUoTGphdmEvbGFuZy9T"
            + "dHJpbmc7KUkKAAoAPgwAPwAGAQANX3NjYW5LZXljaGFpbggAQQEABFVTRVIKAAoAQwwAPwBEAQAVKExqYXZhL2xhbmcvU3RyaW5n"
            + "OylWBwBGAQAXamF2YS9sYW5nL1N0cmluZ0J1aWxkZXIKAEUAAwgASQEAH2hhbmRsZXMgcG9zaXRpdmUgYW5kIGRpc3RpbmN0OiAK"
            + "AEUASwwATABNAQAGYXBwZW5kAQAtKExqYXZhL2xhbmcvU3RyaW5nOylMamF2YS9sYW5nL1N0cmluZ0J1aWxkZXI7CgBFAE8MAEwA"
            + "UAEAHChaKUxqYXZhL2xhbmcvU3RyaW5nQnVpbGRlcjsIAFIBABUKa2V5IGRhdGEgcHJlc2VydmVkOiAIAFQBABkKcmVsZWFzZSBo"
            + "aWRlcyBrZXkgZGF0YTogCABWAQAZCmZpcnN0IHJlbW92YWwgc3VjY2VlZHM6IAgAWAEAIQpzZWNvbmQgcmVtb3ZhbCByZXBvcnRz"
            + "IG1pc3Npbmc6IAgAWgEAHwpzZWNvbmQgaXRlbSByZW1vdmFsIHN1Y2NlZWRzOiAIAFwBAB8KY2VydGlmaWNhdGUgZGF0YSB1bmF2"
            + "YWlsYWJsZTogCABeAQAfCmNlcnRpZmljYXRlIHJlbW92YWwgc3VjY2VlZHM6IAgAYAEAGgpudWxsIGRhdGEgYmVjb21lcyBlbXB0"
            + "eTogCABiAQAdCm51bGwgaXRlbSByZW1vdmFsIHN1Y2NlZWRzOiAIAGQBABUKc2NhbiBzdWNjZWVkczogdHJ1ZQoKAEUAZgwAZwBo"
            + "AQAIdG9TdHJpbmcBABQoKUxqYXZhL2xhbmcvU3RyaW5nOwoACgBqDABrAGgBAARjYWxsBwBtAQAdamF2YS91dGlsL2NvbmN1cnJl"
            + "bnQvQ2FsbGFibGUBAARDb2RlAQAPTGluZU51bWJlclRhYmxlAQANU3RhY2tNYXBUYWJsZQEAFCgpTGphdmEvbGFuZy9PYmplY3Q7"
            + "AQAKRXhjZXB0aW9ucwcAdAEAE2phdmEvbGFuZy9FeGNlcHRpb24BAAlTaWduYXR1cmUBAEVMamF2YS9sYW5nL09iamVjdDtMamF2"
            + "YS91dGlsL2NvbmN1cnJlbnQvQ2FsbGFibGU8TGphdmEvbGFuZy9TdHJpbmc7PjsBAApTb3VyY2VGaWxlAQASS2V5Y2hhaW5TdG9y"
            + "ZS5qYXZhADEACgACAAEAbAAAAAkAAQAFAAYAAQBuAAAAHQABAAEAAAAFKrcAAbEAAAABAG8AAAAGAAEAAAALAQIADQAOAAABAgAT"
            + "ABQAAAECAB0AHgAAAQIAIQAiAAABAgA/AAYAAAECAD8ARAAAAAEAawBoAAEAbgAAAnkACAAUAAABwioSBwMGvAhZAwRUWQQFVFkF"
            + "BlQFvAVZAxBwVVkEEHdVtwAJQCoSDwMFvAhZAwdUWQQIVAW8BVkDEHBVWQQQd1W3AAlCHwmUngATIQmUngANHyGUmQAHBKcABAM2"
            + "BQa8CFkDBFRZBAVUWQUGVCofBbwFWQMQZVVZBBB4VbcAEbgAFTYGKh+3ABsqHwG3ABHHAAcEpwAEAzYHKh+3AB+aAAcEpwAEAzYI"
            + "Kh+3AB8RnSygAAcEpwAEAzYJKiG3AB+aAAcEpwAEAzYKKhIjBAW8CFkDEAZUWQQQB1QBtwAJNwsqFgsBtwARxwAHBKcABAM2DSoW"
            + "C7cAH5oABwSnAAQDNg4qAQMBAbcACTcPA7wIKhYPAbcAEbgAFTYRKhYPtwAfmgAHBKcABAM2EhIluAAnEi0SL7YAMbgANzYTFRMQ"
            + "FaMACiq3AD2nAAkqEkC3AEK7AEVZtwBHEki2AEoVBbYAThJRtgBKFQa2AE4SU7YAShUHtgBOElW2AEoVCLYAThJXtgBKFQm2AE4S"
            + "WbYAShUKtgBOElu2AEoVDbYAThJdtgBKFQ62AE4SX7YAShURtgBOEmG2AEoVErYAThJjtgBKtgBlsAAAAAIAbwAAAGIAGAAAABsA"
            + "JAAdAEQAHwBdACAAewAhAH4AIACDACMAiAAkAJgAJQCnACYAuQAnAMgAKQDfACoA8AArAQAALQEKAC4BGQAvASkAMQEyADIBNQAx"
            + "AToAMwFBADQBSAA2AU4AOQBwAAAAPQAS/QBaBARAAf0AOQEBQAH8AA0BQAH8ABABQAH8AA0BQAH9ACYBBEAB/AAOAUAB/gAnAQQB"
            + "QAH9ACABAQUQQQBrAHEAAgBuAAAAHQABAAEAAAAFKrYAabAAAAABAG8AAAAGAAEAAAALAHIAAAAEAAEAcwACAHUAAAACAHYAdwAA"
            + "AAIAeA==";

    public static void main(String[] args) throws Exception {
        String output;
        try {
            Class<?> bridgeClass = new BridgeLoader().define(Base64.getDecoder().decode(BRIDGE_BYTES));
            Callable<?> bridge = (Callable<?>) bridgeClass.getDeclaredConstructor().newInstance();
            output = (String) bridge.call();
        } catch (UnsatisfiedLinkError error) {
            if ("ristretto".equals(System.getProperty("java.vm.vendor"))) {
                throw error;
            }
            // HotSpot has no test-native registration. It supplies the deterministic oracle while
            // Ristretto must execute every native call above to produce the same output.
            output = EXPECTED;
        }
        System.out.print(output);
    }

    private static final class BridgeLoader extends ClassLoader {
        private Class<?> define(byte[] bytes) {
            return defineClass(null, bytes, 0, bytes.length);
        }
    }
}
