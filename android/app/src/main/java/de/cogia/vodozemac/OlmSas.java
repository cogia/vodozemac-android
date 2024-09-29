package de.cogia.vodozemac;

public class OlmSas {

    private final long ptr;
    private static native long _new();
    private static native String _public_key();

    public OlmSas() {
        ptr =_new();
    }

    public String publicKey() {
        return _public_key();
    }
}
