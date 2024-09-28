package de.cogia.vodozemac;

public class SessionConfig {

    private final long ptr;
    private static native long _version1();
    private static native long _version2();
    private static native long _version(long sessionConfigPtr);

    private SessionConfig(long ptr) {
        this.ptr = ptr;
    }

    public long getPtr() {
        return ptr;
    }

    public static SessionConfig version1() {
        long ptr = _version1();
        return new SessionConfig(ptr);
    }

    public static SessionConfig version2() {
        long ptr = _version2();
        return new SessionConfig(ptr);
    }

    public long version() {
        return _version(ptr);
    }
}
