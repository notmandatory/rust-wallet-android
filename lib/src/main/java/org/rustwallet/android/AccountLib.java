package org.rustwallet.android;

class AccountLib {

    // Used to load the 'rust_wallet_android' library on application startup.
    static {
        System.loadLibrary("rust_wallet_android");
    }

    /**
     * A native methods that are implemented by the 'rust_wallet_android' native library,
     * which is packaged with this application.
     */

    native MasterAccount getMaster(String mnemonic, long birth, int network, String passphrase, String pdPassphrase);

    native Account getAccount(MasterAccount masterAccount, String passphrase, int type, int accountNumber, int subAccountNumber, int seen, int lookahead);

    native String getMnemonic(int entropy);
}
