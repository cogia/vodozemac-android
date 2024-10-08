package de.cogia.vodozemac;

import org.json.JSONException;
import org.json.JSONObject;

import java.util.HashMap;
import java.util.Iterator;

public class OlmAccount {

    private final long ptr;
    private static native long _new();
    private static native IdentityKeys _identity_keys(final long sessionConfigPtr) throws OlmException;
    private static native String _pickle(final long sessionConfigPtr, final String pickleKey) throws OlmException;
    private static native long _from_pickle(final String pickle, final String pickleKey);
    private static native long _from_pickle_lib_olm(final String pickle, final String pickleKey);
    private static native String _ed25519_key(final long ptr);
    private static native String _curve25519Key(final long ptr);
    private static native String _sign(final long ptr, final String message);
    private static native long _maxNumberOfOneTimeKeys(final long ptr);
    private static native String _oneTimeKeys(final long ptr) throws OlmException;
    private static native String _fallbackKey(final long ptr) throws OlmException;
    private static native void  _generateFallbackKey(final long ptr);
    private static native void  _markKeysAsPublished(final long ptr);
    private static native void  _generateOneTimeKeys(final long ptr, final long size);
    private static native long  _createOutboundSession(final long ptr, final String identityKey,
                                                       final String oneTimeKey,
                                                       final long config);
    private static native InboundCreationResult _createInboundSession(final long ptr,
                                                                      final String identityKey,
                                                                      final String chiperText,
                                                                      final long messageType) throws OlmException;

    public OlmAccount() {
        ptr = _new();
    }

    private OlmAccount(final long ptr) {
        this.ptr = ptr;
    }

    public IdentityKeys identityKeys() throws OlmException {
        return _identity_keys(ptr);
    }

    public String pickle(final String pickleKey) throws OlmException {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        return _pickle(ptr, pickleKey);
    }

    public static OlmAccount fromPickle(final String pickle, final String pickleKey) throws OlmException {
        if (pickleKey == null || pickleKey.length() != 32) {
            throw new OlmException("Pickle key must be 32 length");
        }
        long ptr = _from_pickle(pickle, pickleKey);
        return new OlmAccount(ptr);
    }

    public static OlmAccount fromPickleLibOlm(final String pickle, final String pickleKey) {
        long ptr = _from_pickle_lib_olm(pickle, pickleKey);
        return new OlmAccount(ptr);
    }

    public String ed25519Key() {
        return _ed25519_key(ptr);
    }

    public String curve25519Key() {
        return _curve25519Key(ptr);
    }

    public String sign(final String message) {
        return _sign(ptr, message);
    }

    public long maxNumberOfOneTimeKeys() {
        return _maxNumberOfOneTimeKeys(ptr);
    }

    public HashMap<String, String> oneTimeKeys() throws OlmException {
        final String res =  _oneTimeKeys(ptr);
        try {
            final JSONObject obj = new JSONObject(res);
            final HashMap<String, String> map = new HashMap<String, String>();
            final Iterator<String> keysItr = obj.keys();
            while(keysItr.hasNext()) {
                String key = keysItr.next();
                map.put(key, obj.getString(key));
            }
            return map;
        } catch (JSONException err) {
            throw new OlmException(err);
        }

    }

    public void generateOneTimeKeys(final long size) {
        _generateOneTimeKeys(ptr, size);
    }

    public HashMap<String, String> fallbackKey() throws OlmException {
        final String res =  _fallbackKey(ptr);
        try {
            final JSONObject obj = new JSONObject(res);
            final HashMap<String, String> map = new HashMap<String, String>();
            final Iterator<String> keysItr = obj.keys();
            while(keysItr.hasNext()) {
                String key = keysItr.next();
                map.put(key, obj.getString(key));
            }
            return map;
        } catch (JSONException err) {
            throw new OlmException(err);
        }

    }

    public void generateFallbackKey() {
        _generateFallbackKey(ptr);
    }

    public void markKeysAsPublished() {
        _markKeysAsPublished(ptr);
    }

    public OlmSession createOutboundSession(final String identityKey,
                                            final String oneTimeKey,
                                            final SessionConfig config) throws OlmException {
        long sessionPtr = _createOutboundSession(ptr, identityKey, oneTimeKey, config.getPtr());
        return new OlmSession(sessionPtr);
    }

    public InboundCreationResult createInboundSession(final String identityKey, final OlmMessage message) throws OlmException{
        try {
            return _createInboundSession(ptr, identityKey, message.getCiphertext(), message.getMessageType());
        } catch (Throwable t) {
            throw new RuntimeException(t.getMessage());
        }
    }
}
