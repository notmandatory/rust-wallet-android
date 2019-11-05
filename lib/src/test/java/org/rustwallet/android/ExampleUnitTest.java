package org.rustwallet.android;

import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;

import static org.junit.Assert.*;

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * @see <a href="http://d.android.com/tools/testing">Testing documentation</a>
 *
 * Must set environment variable: eg. (on osx)
 * export JAVA_LIBRARY_PATH=[project_home]/lib/src/main/jniLibs/x86_64
 *
**/
public class ExampleUnitTest {

    private static String PASSPHRASE = "correct horse battery staple";

    @Before
    public void before() {
        String userDir = System.getProperty("user.dir");
        System.setProperty("java.library.path", userDir+"/src/main/jniLibs/x86_64");
    }

    @Ignore
    @Test
    public void accountLib_getMaster_notNull() {
        String path = System.getProperty("java.library.path");
        System.out.println(path);

        AccountLib accountLib = new AccountLib();

        MasterAccount master = accountLib.getMaster(32, 1, PASSPHRASE);
        assertNotNull(master);
    }

    @Test
    public void accountService_getMaster_notNull() {
        String path = System.getProperty("java.library.path");
        System.out.println(path);

        AccountService accountService = new AccountService();

        MasterAccount master = accountService.getMaster(Entropy.Low, Network.Bitcoin, PASSPHRASE);
        assertNotNull(master);
    }

    @Test
    public void accountService_getAccount_notNull() {
        String path = System.getProperty("java.library.path");
        System.out.println(path);

        AccountService accountService = new AccountService();

        MasterAccount master = accountService.getMaster(Entropy.Low, Network.Bitcoin, PASSPHRASE);
        assertNotNull(master);

        Account account = accountService.getAccount(master, PASSPHRASE, 84, 0, 0, -1, 10);
        assertNotNull(account);
    }
}