package de.cogia.cogiavodozemacjni;

import android.os.Bundle;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

import de.cogia.vodozemac.IdentityKeys;
import de.cogia.vodozemac.OlmAccount;
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
            IdentityKeys identityKeys = acc.identityKeys();
            IdentityKeys identityKEys = olmAccount.identityKeys();
            //OlmAccount.fromPickle(pickled, null);
            OlmAccount acc2 = OlmAccount.fromPickleLibOlm("pickled", "sdasdsad");
            System.out.println(acc2.identityKeys().getCurve25519());
        } catch (Exception e) {
            System.out.println(e);
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
}