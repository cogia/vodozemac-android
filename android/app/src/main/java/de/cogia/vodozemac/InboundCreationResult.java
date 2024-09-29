package de.cogia.vodozemac;

public class InboundCreationResult {

    private final OlmSession olmSession;
    private final String plainText;

    public InboundCreationResult(final String text, final long sessionPtr) {
        olmSession = new OlmSession(sessionPtr);
        plainText = text;
    }

    public String getPlainText() {
        return plainText;
    }

    public OlmSession getSession() {
        return olmSession;
    }

}
