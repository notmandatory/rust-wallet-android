package org.rustwallet.android;

import org.junit.Ignore;
import org.junit.Test;

import static org.junit.Assert.*;

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * @see <a href="http://d.android.com/tools/testing">Testing documentation</a>
 */
public class ExampleUnitTest {

    private static String PASSPHRASE = "correct horse battery staple";

    // had to set VM options:
    // -Djava.library.path=/Users/steve/git/rust/rust-wallet-android/app/src/main/jniLibs/x86_64
    @Ignore
    @Test
    public void accountLib_getMaster_notNull() {
        String path = System.getProperty("java.library.path");
        System.out.println(path);

        AccountLib accountLib = new AccountLib();

        String master = accountLib.getMaster(32, 0x0709110B, PASSPHRASE);
        assertNotNull(master);
    }
}