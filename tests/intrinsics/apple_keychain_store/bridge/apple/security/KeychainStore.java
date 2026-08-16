package apple.security;

import java.util.Arrays;
import java.util.concurrent.Callable;

/**
 * Test bridge whose binary name matches the macOS JDK class that owns the native methods.
 * Test.java embeds the Java 8 class file generated from this source so the bridge can be loaded
 * without conflicting with the real package-private KeychainStore class on macOS.
 */
public final class KeychainStore implements Callable<String> {
    private native long _addItemToKeychain(
            String alias, boolean certificate, byte[] data, char[] password);

    private native byte[] _getEncodedKeyData(long keychainItemRef, char[] password);

    private native void _releaseKeychainItemRef(long keychainItemRef);

    private native int _removeItemFromKeychain(long keychainItemRef);

    private native void _scanKeychain();

    private native void _scanKeychain(String storeName);

    @Override
    public String call() {
        long first = _addItemToKeychain("first", false, new byte[] {1, 2, 3},
                new char[] {'p', 'w'});
        long second = _addItemToKeychain("second", false, new byte[] {4, 5},
                new char[] {'p', 'w'});
        boolean handlesAreDistinct = first > 0 && second > 0 && first != second;
        boolean keyDataPreserved = Arrays.equals(
                new byte[] {1, 2, 3}, _getEncodedKeyData(first, new char[] {'e', 'x'}));

        _releaseKeychainItemRef(first);
        boolean releasedKeyIsHidden = _getEncodedKeyData(first, null) == null;
        boolean firstRemovalSucceeded = _removeItemFromKeychain(first) == 0;
        boolean missingItemReported = _removeItemFromKeychain(first) == -25300;
        boolean secondRemovalSucceeded = _removeItemFromKeychain(second) == 0;

        long certificate = _addItemToKeychain("certificate", true, new byte[] {6, 7}, null);
        boolean certificateDataIsHidden = _getEncodedKeyData(certificate, null) == null;
        boolean certificateRemovalSucceeded = _removeItemFromKeychain(certificate) == 0;

        long empty = _addItemToKeychain(null, false, null, null);
        boolean nullDataBecomesEmpty = Arrays.equals(new byte[0], _getEncodedKeyData(empty, null));
        boolean emptyRemovalSucceeded = _removeItemFromKeychain(empty) == 0;

        int javaVersion = Integer.parseInt(System.getProperty("java.specification.version")
                .replace("1.", ""));
        if (javaVersion <= 21) {
            _scanKeychain();
        } else {
            _scanKeychain("USER");
        }

        return "handles positive and distinct: " + handlesAreDistinct + "\n"
                + "key data preserved: " + keyDataPreserved + "\n"
                + "release hides key data: " + releasedKeyIsHidden + "\n"
                + "first removal succeeds: " + firstRemovalSucceeded + "\n"
                + "second removal reports missing: " + missingItemReported + "\n"
                + "second item removal succeeds: " + secondRemovalSucceeded + "\n"
                + "certificate data unavailable: " + certificateDataIsHidden + "\n"
                + "certificate removal succeeds: " + certificateRemovalSucceeded + "\n"
                + "null data becomes empty: " + nullDataBecomesEmpty + "\n"
                + "null item removal succeeds: " + emptyRemovalSucceeded + "\n"
                + "scan succeeds: true\n";
    }
}
