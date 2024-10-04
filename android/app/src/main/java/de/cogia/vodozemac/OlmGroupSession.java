package de.cogia.vodozemac;

public class OlmGroupSession {
    private final long ptr;

    private static native long _new(final long ptr);
    private static native String _session_id(final long ptr);
    private static native String _session_key(final long ptr);
    private static native String _message_key(final long ptr);
    private static native String _encrypt(final long ptr, final String message);
    private static native String _pickle(final long ptr, final String passPhrase) throws OlmException;
    private static native long _from_pickle(final String pickle, final String passPhrase);

    public OlmGroupSession(final SessionConfig config) {
        ptr = _new(config.getPtr());
    }

    private OlmGroupSession(final long ptr) {
        this.ptr = ptr;
    }

    public String sessionId() {
        return _session_id(ptr);
    }

    public String sessionKey() {
        return _session_key(ptr);
    }

    public String messageKey() {
        return _message_key(ptr);
    }

    public String encrypt(final String message) {
        return _encrypt(ptr, message);
    }

    public String pickle(final String passPhrase) throws OlmException {
        if (passPhrase == null || passPhrase.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        return  _pickle(ptr, passPhrase);
    }

    public static OlmGroupSession fromPickle(final String pickle, final String passPhrase) throws OlmException {
        if (passPhrase == null || passPhrase.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        final long ptr = _from_pickle(pickle, passPhrase);
        return new OlmGroupSession(ptr);
    }

}
