package org.rustwallet.android;

import java.util.Arrays;
import java.util.Date;
import java.util.List;

public class AccountService {

    /**
     * Friendlier public facing methods to access AccountLib native JNI functions.
     */

    private final AccountLib accountLib;

    public AccountService() {
        accountLib = new AccountLib();
    }

    public MasterAccount getMaster(List<String> mnemonicWords, Date birth, Network network, String passphrase, String pdPassphrase) {
        StringBuilder sb = new StringBuilder();
        for (String n : mnemonicWords) {
            if (sb.length() > 0) sb.append(' ');
            sb.append(n);
        }
        MasterAccount master = accountLib.getMaster(sb.toString(), birth.getTime(), network.ordinal(), passphrase, pdPassphrase);
        return master;
    }

    public Account getAccount(MasterAccount master, String passphrase, int type, int accountNumber, int subAccountNumber, int seen, int lookahead) {
        Account account = accountLib.getAccount(master, passphrase, type, accountNumber, subAccountNumber, seen, lookahead);
        return account;
    }

    public List<String> getMnemonicWords(Entropy entropy) {
        String mnemonic = accountLib.getMnemonic(entropy.n);
        List<String> mnemonicWords = Arrays.asList(mnemonic.split(" "));
        return mnemonicWords;
    }
}
