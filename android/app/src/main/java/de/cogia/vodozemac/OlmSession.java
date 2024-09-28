package de.cogia.vodozemac;

public class OlmSession {

    private final long ptr;

    private static native String _pickle(final long sessionConfigPtr, final String pickleKey);
    private static native long _from_pickle(final String pickle, final String pickleKey);
    private static native long _from_pickle_lib_olm(final String pickle, final String pickleKey);
    private static native String _session_id(final long ptr);
    private static native boolean _session_matches(final long ptr, final String text, final long type);
    private static native String _decrypt(final long ptr,  final String text, final long type);
    private static native OlmMessage _encrypt(final long ptr,  final String text);


    public OlmSession(final long ptr) {
        this.ptr = ptr;
    }

    public String pickle(final String pickleKey) throws Exception {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new Exception("Pickle key must be 32 length");
        }
        return _pickle(ptr, pickleKey);
    }

    public static OlmSession fromPickle(final String pickle, final String pickleKey) throws Exception {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new Exception("Pickle key must be 32 length");
        }
        long ptr = _from_pickle(pickle, pickleKey);
        return new OlmSession(ptr);
    }

    public static OlmSession fromPickleLibOlm(final String pickle, final String pickleKey) {
        long ptr = _from_pickle_lib_olm(pickle, pickleKey);
        return new OlmSession(ptr);
    }

    public String sessionId() {
        return _session_id(ptr);
    }

    public boolean sessionMatches(final OlmMessage message) {
        return _session_matches(ptr, message.getCiphertext(), message.getMessageType());
    }

    public String decrypt(final OlmMessage message) {
        return _decrypt(ptr, message.getCiphertext(), message.getMessageType());
    }

    public OlmMessage encrypt(final String message) {
        return _encrypt(ptr, message);
    }
}
