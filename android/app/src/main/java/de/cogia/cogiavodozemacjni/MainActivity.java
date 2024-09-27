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
        IdentityKeys identityKEys = olmAccount.identityKeys();
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