use bitcoin_wallet::account::{Account, AccountAddressType, MasterAccount, Unlocker};

pub fn get_account(master: &MasterAccount, passphrase: &str, address_type: AccountAddressType, account_number: u32, sub_account_number: u32,
                   seen: u32, lookahead: u32) -> Account {

    // create an unlocker that is able to decrypt the encrypted mnemonic and then calculate private keys
    let mut unlocker = Unlocker::new_for_master(&master, passphrase).unwrap();

    // create an account for the specified address type
    let mut account = Account::new(&mut unlocker, address_type, account_number, sub_account_number, lookahead).unwrap();
    account.do_look_ahead(Some(seen)).unwrap();
    account
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;
    use bitcoin_wallet::account::{MasterAccount, MasterKeyEntropy};
    use bitcoin_wallet::account::AccountAddressType::P2WPKH;
    use bitcoin_wallet::mnemonic::Mnemonic;

    use crate::account::get_account;

    const ENTROPY: MasterKeyEntropy = MasterKeyEntropy::Low;
    const NETWORK: Network = Network::Bitcoin;
    const WORDS: &str = "announce damage viable ticket engage curious yellow ten clock finish burden orient faculty rigid smile host offer affair suffer slogan mercy another switch park";
    const PASSPHRASE: &str = "correct horse battery staple";

    // get first receive address, m / 84'/0'/0'/0 / 0, verify 10 addresses instantiated
    #[test]
    fn test_get_account_0() {
        let mnemonic = Mnemonic::from_str(WORDS).unwrap();
        let mut master = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, None).unwrap();
        let account = get_account(&mut master, PASSPHRASE, P2WPKH, 0, 0, 0, 10);
        let receive_address_0 = account.get_key(0).unwrap().address.to_string();
        assert_eq!(receive_address_0, "bc1qlz2h9scgalmqj43d36f58dcxrrl7udu999gcp2");
        assert_eq!(account.instantiated().len(), 10);
    }

    // get first and second receive address, m / 84'/0'/0'/0 / 0..1, verify 11 addresses instantiated
    #[test]
    fn test_get_account_1() {
        let mnemonic = Mnemonic::from_str(WORDS).unwrap();
        let mut master = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, None).unwrap();
        let account = get_account(&mut master, PASSPHRASE, P2WPKH, 0, 0, 1, 10);
        let receive_address_0 = account.get_key(0).unwrap().address.to_string();
        let receive_address_1 = account.get_key(1).unwrap().address.to_string();
        assert_eq!(receive_address_0, "bc1qlz2h9scgalmqj43d36f58dcxrrl7udu999gcp2");
        assert_eq!(receive_address_1, "bc1q0ma7jqn8rdz9kalemd4aszguc66g262qm02e2e");
        assert_eq!(account.instantiated().len(), 11);
    }
}