package de.cogia.vodozemac;

public class OlmInboundGroupSession {

    private final long ptr;

    private static native long _new(final String sessionKey, final long ptr);
    private static native long _import(final String sessionKey, final long ptr) throws OlmException;
    private static native String _pickle(final long ptr, final String passPhrase) throws OlmException;
    private static native long _from_pickle(final String pickle, final String passPhrase);
    private static native long _from_libolm_pickle(final String pickle, final String passPhrase);
    private static native String _session_id(final long ptr);
    private static native long _first_known_index(final long ptr);
    private static native String _export_at(final long ptr, final long index);
    private static native OlmDecryptedMessage _decrypt(final long ptr, final String cipertext) throws OlmException;

    public OlmInboundGroupSession(final String sessionKey, final SessionConfig config) {
        this.ptr = _new(sessionKey, config.getPtr());
    }

    private OlmInboundGroupSession(final long ptr) {
        this.ptr = ptr;
    }

    public String pickle(final String passPhrase) throws OlmException {
        if (passPhrase == null || passPhrase.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        return  _pickle(ptr, passPhrase);
    }

    public static OlmInboundGroupSession fromPickle(final String pickle, final String passPhrase) throws OlmException {
        if (passPhrase == null || passPhrase.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        final long ptr = _from_pickle(pickle, passPhrase);
        return new OlmInboundGroupSession(ptr);
    }

    public String sessionId() {
        return _session_id(ptr);
    }

    public long firstKnownIndex() {
        return _first_known_index(ptr);
    }

    public static OlmInboundGroupSession importFrom(final String sessionKey, final SessionConfig config) throws OlmException {
        final long ptr = _import(sessionKey, config.getPtr());
        return new OlmInboundGroupSession(ptr);
    }

    public String exportAt(final long index) {
        return _export_at(ptr, index);
    }

    public static OlmInboundGroupSession fromLibOlmPickle(final String pickle, final String passPhrase) throws OlmException {
        if (passPhrase == null || passPhrase.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        final long ptr =  _from_libolm_pickle(pickle, passPhrase);
        return new OlmInboundGroupSession(ptr);
    }

    public OlmDecryptedMessage decrypt(final String cipertext) throws OlmException {
        return _decrypt(ptr, cipertext);
    }
}
