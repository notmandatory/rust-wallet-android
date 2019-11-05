package org.rustwallet.android;

import java.util.Arrays;
import java.util.List;

/**
 * struct AccountExt {
 * address_type: u32,
 * account_number: u32,
 * sub_account_number: u32,
 * instantiated: Vec<String>,
 * next: u32,
 * network: String,
 * }
 **/

public class Account {

    private int addressType;

    private int accountNumber;

    private int subAccountNumber;

    private List<String> instantiated;

    private int next;

    private Network network;

    public Account() {
    }

    public Account(int addressType, int accountNumber, int subAccountNumber, String[] instantiated, int next, int network) {
        this.addressType = addressType;
        this.accountNumber = accountNumber;
        this.subAccountNumber = subAccountNumber;
        this.instantiated = Arrays.asList(instantiated);
        this.next = next;
        this.network = Network.values()[network];
    }

    public int getAddressType() {
        return addressType;
    }

    public int getAccountNumber() {
        return accountNumber;
    }

    public int getSubAccountNumber() {
        return subAccountNumber;
    }

    public List<String> getInstantiated() {
        return instantiated;
    }

    public int getNext() {
        return next;
    }

    public Network getNetwork() {
        return network;
    }
}
