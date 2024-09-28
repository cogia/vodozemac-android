package de.cogia.vodozemac;

public class InboundCreationResult {

    private final OlmSession olmSession;
    private final String plainText;

    public InboundCreationResult(final long sessionPtr, final String plainText) {
        this.olmSession = new OlmSession(sessionPtr);
        this.plainText = plainText;
    }

    public String getPlainText() {
        return plainText;
    }

    public OlmSession getSession() {
        return olmSession;
    }

}
