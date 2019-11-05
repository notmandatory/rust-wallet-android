package org.rustwallet.android;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import org.rustwallet.android.gson.DateTypeAdapter;
import org.rustwallet.android.gson.NetworkTypeAdapter;

import java.util.Date;

public class AccountService {

    /**
     * Friendlier public facing methods to access AccountLib native JNI functions.
     */

    private final AccountLib accountLib;

    public AccountService() {
        accountLib = new AccountLib();
    }

    public MasterAccount getMaster(Entropy entropy, Network network, String passphrase) {
        MasterAccount master = accountLib.getMaster(entropy.n, network.ordinal(), passphrase);
        return master;
    }

    public Account getAccount(MasterAccount master, String passphrase, int type, int accountNumber, int subAccountNumber, int seen, int lookahead) {
        Account account = accountLib.getAccount(master, passphrase, type, accountNumber, subAccountNumber, seen, lookahead);
        return account;
    }
}
