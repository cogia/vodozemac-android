package de.cogia.vodozemac;

public class OlmAccount {

    private final long ptr;
    private static native long _new();
    private static native IdentityKeys _identity_keys(long sessionConfigPtr);

    public OlmAccount() {
        ptr = _new();
    }

    public IdentityKeys identityKeys() {
        return _identity_keys(ptr);
    }
}
