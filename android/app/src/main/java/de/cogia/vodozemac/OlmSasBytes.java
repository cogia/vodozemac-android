package de.cogia.vodozemac;

public class OlmSasBytes {

    private final long ptr;
    private static native long _bytes(final long ptr, final String info);

    public OlmSasBytes(final long ptr) {
        this.ptr = ptr;
    }

}
