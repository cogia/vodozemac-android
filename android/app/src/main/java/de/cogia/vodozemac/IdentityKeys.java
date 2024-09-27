package de.cogia.vodozemac;

public class IdentityKeys {

    private String ed25519;
    private String curve25519;

    public IdentityKeys(String ed25519, String curve25519) {
        this.ed25519 = ed25519;
        this.curve25519 = curve25519;
    }

    public String getEd25519() {
        return ed25519;
    }

    public String getCurve25519() {
        return curve25519;
    }
}
