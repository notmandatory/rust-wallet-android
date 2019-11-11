package org.rustwallet.android;

import org.junit.Test;

import java.util.Arrays;
import java.util.Date;
import java.util.List;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertNotNull;

/**
 * AccountService local unit test, which will execute on the development machine (host).
 *
 * @see <a href="http://d.android.com/tools/testing">Testing documentation</a>
 * <p>
 * Must set environment variable: eg. (on osx)
 * export JAVA_LIBRARY_PATH=[project_home]/lib/src/main/jniLibs/x86_64
 **/
public class AccountServiceTest {

    private static String PASSPHRASE = "correct horse battery staple";
    private static String PD_PASSPHRASE_1 = "test123";
    private static String PD_PASSPHRASE_2 = "123test";

    private static List<String> MNEMONIC_WORDS = Arrays.asList("reason","since","adjust","settle","menu","auction","material","beyond","bomb","repair","appear","length");
    private static String XPUB_KEY = "xpub661MyMwAqRbcGCNhNJWLH3CdBsVTo97ZVU8os3QhczR4R3ddekGD2e1PCWw15hnuRYoAnnz842ff9NCr35w3ZyF9FZC9nrbcJJbeGZxFAR8";

    private final AccountService accountService;

    public AccountServiceTest() {
        accountService = new AccountService();
    }

    @Test
    public void accountLib_getMnemonic_notNull() {
        List<String> mnemonicWords = accountService.getMnemonicWords(Entropy.Low);
        assertNotNull(mnemonicWords);
    }

    @Test
    public void accountService_getMaster_notNull() {
        MasterAccount master = accountService.getMaster(MNEMONIC_WORDS, new Date(), Network.Bitcoin, PASSPHRASE, null);
        assertNotNull(master);
        assertEquals(XPUB_KEY, master.getMasterPublic());
    }

    @Test
    public void accountService_getMaster_withSamePdPassphrase() {
        Date date = new Date();

        MasterAccount master1 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, PD_PASSPHRASE_1);

        MasterAccount master2 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, PD_PASSPHRASE_1);

        assertEquals(master1.getMasterPublic(), master2.getMasterPublic());

        MasterAccount master3 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, null);

        MasterAccount master4 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, null);

        assertEquals(master3.getMasterPublic(), master4.getMasterPublic());
    }

    @Test
    public void accountService_getMaster_withDifferentPdPassphrase() {
        Date date = new Date();

        MasterAccount master1 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, PD_PASSPHRASE_1);

        MasterAccount master2 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, null);

        assertNotEquals(master1.getMasterPublic(), master2.getMasterPublic());

        MasterAccount master3 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, PD_PASSPHRASE_1);

        MasterAccount master4 = accountService.getMaster(MNEMONIC_WORDS, date, Network.Bitcoin, PASSPHRASE, PD_PASSPHRASE_2);

        assertNotEquals(master3.getMasterPublic(), master4.getMasterPublic());
    }

    @Test
    public void accountService_getAccount_notNull() {
        MasterAccount master = accountService.getMaster(MNEMONIC_WORDS, new Date(), Network.Bitcoin, PASSPHRASE, null);
        assertNotNull(master);

        Account account = accountService.getAccount(master, PASSPHRASE, 84, 0, 0, -1, 10);
        assertNotNull(account);
    }
}