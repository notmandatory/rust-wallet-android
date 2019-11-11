package org.rustwallet.android;

import java.util.Date;

/**
 * struct MasterAccountExt {
 * master_public: ExtendedPubKey,
 * encrypted: Vec<u8>,
 * birth: u64,
 * network: Network,
 * }
 */
public class MasterAccount {

    private String masterPublic;

    private byte[] encrypted;

    private Date birth;

    public MasterAccount() {
    }

    public MasterAccount(String masterPublic, byte[] encrypted, long birth) {
        this.masterPublic = masterPublic;
        this.encrypted = encrypted;
        this.birth = new Date(birth);
    }

    public String getMasterPublic() {
        return masterPublic;
    }

    public byte[] getEncrypted() {
        return encrypted;
    }

    public Date getBirth() {
        return birth;
    }
}
