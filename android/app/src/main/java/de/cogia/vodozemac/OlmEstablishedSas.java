package de.cogia.vodozemac;

public class OlmEstablishedSas {

    private final long ptr;

    private static native long _bytes(final long ptr, final String info);
    private static native String _calculate_mac(final long ptr, final String input, final String info);
    private static native String _calculate_mac_invalid_base64(final long ptr, final String input, final String info);
    private static native boolean _verify_mac(final long ptr, final String input, final String info, final String tag);


    public OlmEstablishedSas(final long ptr) {
        this.ptr = ptr;
    }

    public OlmSasBytes bytes(final String info) {
        return new OlmSasBytes(_bytes(ptr, info));
    }


    public String calculateMac(final String input, final String info) {
        return _calculate_mac(ptr, input, info);
    }

    public String calculateMacInvalidBase64(final String input, final String info) {
        return _calculate_mac_invalid_base64(ptr, input, info);
    }

    public boolean verifyMac(final String input, final String info, final String tag) {
        return _verify_mac(ptr, input, info, tag);
    }

}
