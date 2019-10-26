package org.rustwallet.android;

import android.os.Bundle;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {

    private static String PASSPHRASE = "correct horse battery staple";


    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        TextView tv = findViewById(R.id.receive_addr);

        AccountLib accountLib = new AccountLib();

        String master = accountLib.getMaster(32, 0x0709110B, PASSPHRASE);

        String account = accountLib.getAccount(master, PASSPHRASE, 0, 0, 0, 10);

        tv.setText(account);
    }
}
