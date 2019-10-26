use std::convert::TryFrom;

use bitcoin::Network;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin_wallet::account::{Account, AccountAddressType, MasterAccount, Unlocker};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MasterAccountExt {
    master_public: ExtendedPubKey,
    encrypted: Vec<u8>,
    birth: u64,
    network: Network,
}

impl MasterAccountExt {
    fn new(master: &MasterAccount) -> MasterAccountExt {
        MasterAccountExt {
            master_public: master.master_public().clone(),
            encrypted: master.encrypted().clone(),
            birth: master.birth(),
            network: master.master_public().network,
        }
    }
}

pub fn master_to_json(master: &MasterAccount) -> String {
    let backup = MasterAccountExt::new(master);
    serde_json::to_string(&backup).unwrap()
}

pub fn master_from_json(master_json: &str) -> MasterAccount {
    let master_ext: MasterAccountExt = serde_json::from_str(master_json).unwrap();
    MasterAccount::from_encrypted(&master_ext.encrypted, master_ext.master_public, master_ext.birth)
}


#[derive(Serialize, Deserialize)]
struct AccountExt {
    address_type: u32,
    account_number: u32,
    sub_account_number: u32,
    instantiated: Vec<String>,
    next: u32,
    network: u32,
}

impl AccountExt {
    fn new(account: &Account) -> AccountExt {
        let instantiated = account.instantiated().iter()
            // return instantiated keys as address base58 strings
            .map(|i| i.address.to_string()).collect();

        AccountExt {
            address_type: account.address_type().as_u32(),
            account_number: account.account_number(),
            sub_account_number: account.sub_account_number(),
            instantiated,
            next: u32::try_from(account.next()).unwrap(),
            network: account.network().magic(),
        }
    }
}

pub fn get_account(master: &MasterAccount, passphrase: &str, address_type: AccountAddressType, account_number: u32, sub_account_number: u32,
                   seen: u32, lookahead: u32) -> Account {

    // create an unlocker that is able to decrypt the encrypted mnemonic and then calculate private keys
    let mut unlocker = Unlocker::new_for_master(&master, passphrase).unwrap();

    // create an account for the specified address type
    let mut account = Account::new(&mut unlocker, address_type, account_number, sub_account_number, lookahead).unwrap();
    account.do_look_ahead(Some(seen)).unwrap();
    account
}

pub fn account_to_json(account: &Account) -> String {
    let account_ext = AccountExt::new(&account);
    serde_json::to_string(&account_ext).unwrap()
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;
    use bitcoin_wallet::account::{MasterAccount, MasterKeyEntropy};
    use bitcoin_wallet::account::AccountAddressType::P2WPKH;
    use bitcoin_wallet::mnemonic::Mnemonic;

    use crate::account::{account_to_json, AccountExt, get_account, master_from_json, master_to_json};

    const ENTROPY: MasterKeyEntropy = MasterKeyEntropy::Low;
    const NETWORK: Network = Network::Bitcoin;
    const WORDS: &str = "announce damage viable ticket engage curious yellow ten clock finish burden orient faculty rigid smile host offer affair suffer slogan mercy another switch park";
    const PASSPHRASE: &str = "correct horse battery staple";

    // encode master account to json backup and restore from json backup
    #[test]
    fn test_master_to_from_json() {
        let master = MasterAccount::new(ENTROPY, NETWORK, PASSPHRASE).unwrap();
        let master_json = master_to_json(&master);
        let master_restored = master_from_json(&master_json);

        assert_eq!(master_restored.master_public(), master.master_public());
        assert_eq!(master_restored.encrypted(), master.encrypted());
        assert_eq!(master_restored.birth(), master.birth());
        assert_eq!(master_restored.seed(NETWORK, PASSPHRASE).unwrap(), master.seed(NETWORK, PASSPHRASE).unwrap());
    }

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

    #[test]
    fn test_account_backup_to_json() {
        let mnemonic = Mnemonic::from_str(WORDS).unwrap();
        let mut master = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, None).unwrap();
        let account = get_account(&mut master, PASSPHRASE, P2WPKH, 0, 0, 1, 10);
        let account_json = account_to_json(&account);
        println!("account_json: {}", account_json);
        let account_restored: AccountExt = serde_json::from_str(&account_json).unwrap();
        assert_eq!(account_restored.address_type, 84);
        assert_eq!(account_restored.instantiated.get(0).unwrap(), "bc1qlz2h9scgalmqj43d36f58dcxrrl7udu999gcp2");
    }
}