package de.cogia.vodozemac;

public class OlmSasBytes {

    private final long ptr;
    private static native long[] _emoji_indices(final long ptr);
    private static native long[] _decimals(final long ptr);

    public OlmSasBytes(final long ptr) {
        this.ptr = ptr;
    }

    public long[] decimals() {
        return _decimals(ptr);
    }

    public long[] emoji_indices() {
        return _emoji_indices(ptr);
    }

}
