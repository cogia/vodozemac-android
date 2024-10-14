package de.cogia.cogiavodozemacjni;

import android.os.Bundle;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

import org.json.JSONException;

import de.cogia.vodozemac.IdentityKeys;
import de.cogia.vodozemac.InboundCreationResult;
import de.cogia.vodozemac.OlmAccount;
import de.cogia.vodozemac.OlmException;
import de.cogia.vodozemac.OlmMessage;
import de.cogia.vodozemac.OlmSession;
import de.cogia.vodozemac.SessionConfig;

public class MainActivity extends AppCompatActivity {


    static {
        System.loadLibrary("vodozemac_android");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {

        SessionConfig sessionConfig = SessionConfig.version1();
        System.out.println(sessionConfig.version());
        System.out.println(SessionConfig.version2().version());

        OlmAccount olmAccount = new OlmAccount();
        try {
            String pickled = olmAccount.pickle("63482006333619702407533275865961");
            System.out.println(pickled);
            OlmAccount acc = OlmAccount.fromPickle(pickled, "63482006333619702407533275865961");
            System.out.println(acc.curve25519Key());
            System.out.println(acc.ed25519Key());
            System.out.println(acc.sign("sdsdsd"));
            System.out.println(acc.maxNumberOfOneTimeKeys());
            acc.generateOneTimeKeys(50);
            acc.generateOneTimeKeys(1111);
            System.out.println(acc.oneTimeKeys());
            acc.generateFallbackKey();
            acc.markKeysAsPublished();
            System.out.println(acc.fallbackKey());
            IdentityKeys identityKeys = acc.identityKeys();
            IdentityKeys identityKEys = olmAccount.identityKeys();
            //OlmAccount.fromPickle(pickled, null);
            OlmAccount acc2 = OlmAccount.fromPickleLibOlm("pickled", "sdasdsad");
            System.out.println(acc2.identityKeys().getCurve25519());
        } catch (Exception e) {
            System.out.println(e);
        }

        try {
            testEncryption();
        } catch (Exception e) {
            throw new RuntimeException(e);
        }

        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_main);
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main), (v, insets) -> {
            Insets systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom);
            return insets;
        });
    }
    public static void testEncryption() throws JSONException, OlmException {
        OlmAccount alice = new OlmAccount();
        OlmAccount bob = new OlmAccount();

        bob.generateOneTimeKeys(4);

        String[] bobOnetimeKeys = bob.oneTimeKeys().values().toArray(new String[0]);
        String bobFirstOnetimeKey = bobOnetimeKeys[0];

        OlmSession session = alice.createOutboundSession(bob.curve25519Key(), bobFirstOnetimeKey, SessionConfig.version2());
        System.out.println(session.sessionId());
        OlmMessage res = session.encrypt("Hello there");
        InboundCreationResult iRes = bob.createInboundSession(alice.curve25519Key(), res);
        //InboundCreationResult iRes2 = bob.createInboundSession(alice.curve25519Key(), res);

        System.out.println(iRes.getPlainText() ==  "Hello there");

        OlmMessage message = iRes.getSession().encrypt("ddddd");


        try {
            String decrypted2 = session.decrypt(new OlmMessage("asdasdasd", 0));
            System.out.println(message.getCiphertext() == decrypted2);
        } catch (Exception e) {
            System.out.println(e);
        }

        try {
            boolean decrypted2 = session.sessionMatches(new OlmMessage("asdasdasd", 0));
        } catch (Exception e) {
            System.out.println(e);
        }


        // one time key removes on first usage
        //t.false(isEqual(bobOnetimeKeys, Object.values(bob.oneTimeKeys)))
        //t.false(Object.values(bob.oneTimeKeys).includes(bobFirstOnetimeKey))*/
    }
}