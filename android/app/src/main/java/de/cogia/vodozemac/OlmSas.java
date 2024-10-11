package de.cogia.vodozemac;

public class OlmSas {

    private final long ptr;
    private static native long _new();
    private static native String _public_key();
    private static native long _diffie_hellman(final long ptr, final String key) throws OlmException;

    public OlmSas() {
        ptr =_new();
    }

    public String publicKey() {
        return _public_key();
    }

    public OlmEstablishedSas diffie_hellman(final String key) throws OlmException {
        final long res = _diffie_hellman(ptr, key);
        if (res == -1) {
            throw new OlmException("failed to diffie_hellman");
        }
        return new OlmEstablishedSas(res);
    }
}
