package org.rustwallet.android;

public class AccountLib {

    // Used to load the 'rust_wallet_android' library on application startup.
    static {
        System.loadLibrary("rust_wallet_android");
    }

    /**
     * A native methods that are implemented by the 'rust_wallet_android' native library,
     * which is packaged with this application.
     */

    public native String getMaster(int entropy, int network, String passphrase);

    public native String getAccount(String masterJson, String passphrase, int accountNumber, int subAccountNumber, int seen, int lookahead);

}
