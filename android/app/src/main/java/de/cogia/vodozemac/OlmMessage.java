package de.cogia.vodozemac;

public class OlmMessage {

    private final long messageType;
    private final String ciphertext;

    public OlmMessage(String ciphertext, long message_type) {
        this.ciphertext = ciphertext;
        this.messageType = message_type;
    }

    public String getCiphertext() {
        return this.ciphertext;
    }

    public long getMessageType() {
        return this.messageType;
    }
}
