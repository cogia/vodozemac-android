package de.cogia.vodozemac;

public class OlmAccount {

    private final long ptr;
    private static native long _new();
    private static native IdentityKeys _identity_keys(final long sessionConfigPtr);
    private static native String _pickle(final long sessionConfigPtr, final String pickleKey);
    private static native long _from_pickle(final String pickle, final String pickleKey);

    public OlmAccount() {
        ptr = _new();
    }

    public OlmAccount(final long ptr) {
        this.ptr = ptr;
    }

    public IdentityKeys identityKeys() {
        return _identity_keys(ptr);
    }

    public String pickle(final String pickleKey) throws Exception {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new Exception("Pickle key must be 32 length");
        }
        return _pickle(ptr, pickleKey);
    }

    public static OlmAccount fromPickle(final String pickle, final String pickleKey) throws Exception {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new Exception("Pickle key must be 32 length");
        }
        long ptr = _from_pickle(pickle, pickleKey);
        return new OlmAccount(ptr);
    }
}
