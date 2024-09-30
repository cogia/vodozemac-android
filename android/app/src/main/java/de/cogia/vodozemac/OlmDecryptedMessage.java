package de.cogia.vodozemac;

public class OlmDecryptedMessage {

    private final String message;
    private final long messageNumber;

    public OlmDecryptedMessage(final String message, final long messageNumber) {
        this.message = message;
        this.messageNumber = messageNumber;
    }

    public String getMessage() {
        return message;
    }

    public long getMessageNumber() {
        return messageNumber;
    }
}
