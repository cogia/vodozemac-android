package de.cogia.vodozemac;

public class OlmException extends Exception {
    public OlmException(String message) {
        super(message);
    }

    public OlmException(Throwable throwable) {
        super(throwable);
    }
}
