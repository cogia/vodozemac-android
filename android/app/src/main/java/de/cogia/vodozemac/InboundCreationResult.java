package de.cogia.vodozemac;

public class InboundCreationResult {

    private final Session session;
    private final String plainText;

    public InboundCreationResult(final long sessionPtr, final String plainText) {
        this.session = new Session(sessionPtr);
        this.plainText = plainText;
    }

    public String getPlainText() {
        return plainText;
    }

    public Session getSession() {
        return session;
    }

}
