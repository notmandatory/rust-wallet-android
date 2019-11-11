use bitcoin_wallet::account::{Account, AccountAddressType, MasterAccount, MasterKeyEntropy, Seed, Unlocker};
use bitcoin_wallet::mnemonic::Mnemonic;
use rand::{RngCore, thread_rng};

pub fn get_account(master: &MasterAccount, passphrase: &str, address_type: AccountAddressType, account_number: u32, sub_account_number: u32,
                   seen: u32, lookahead: u32) -> Account {

    // create an unlocker that is able to decrypt the encrypted mnemonic and then calculate private keys
    let mut unlocker = Unlocker::new_for_master(&master, passphrase).unwrap();

    // create an account for the specified address type
    let mut account = Account::new(&mut unlocker, address_type, account_number, sub_account_number, lookahead).unwrap();
    account.do_look_ahead(Some(seen)).unwrap();
    account
}

pub fn new_mnemonic(entropy: MasterKeyEntropy) -> Mnemonic {
    let mut random = vec!(0u8; entropy as usize);
    thread_rng().fill_bytes(random.as_mut_slice());
    ;
    let seed = Seed(random);

    let mnemonic = Mnemonic::new(seed.0.as_slice())
        .expect("error creating Mnemonic::new");
    mnemonic
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::Network;
    use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
    use bitcoin_wallet::account::{MasterAccount, MasterKeyEntropy};
    use bitcoin_wallet::account::AccountAddressType::P2WPKH;
    use bitcoin_wallet::mnemonic::Mnemonic;
    use secp256k1::{self, Secp256k1};

    use crate::account::{get_account, new_mnemonic};

    const ENTROPY: MasterKeyEntropy = MasterKeyEntropy::Low;
    const NETWORK: Network = Network::Bitcoin;
    const WORDS: &str = "announce damage viable ticket engage curious yellow ten clock finish burden orient faculty rigid smile host offer affair suffer slogan mercy another switch park";
    const PASSPHRASE: &str = "correct horse battery staple";
    const PD_PASSPHRASE: Option<&str> = Some("test123");

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
    fn test_new_mnemonic() {
        let mnemonic = new_mnemonic(ENTROPY);

        let master1 = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, None).unwrap();
        let master2 = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, None).unwrap();

        assert_eq!(master1.seed(NETWORK, PASSPHRASE).unwrap(), master2.seed(NETWORK, PASSPHRASE).unwrap());

        let master3 = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, PD_PASSPHRASE).unwrap();
        let master4 = MasterAccount::from_mnemonic(&mnemonic, 0, NETWORK, PASSPHRASE, PD_PASSPHRASE).unwrap();

        assert_eq!(master3.seed(NETWORK, PASSPHRASE).unwrap(), master4.seed(NETWORK, PASSPHRASE).unwrap());
        assert_ne!(master1.seed(NETWORK, PASSPHRASE).unwrap(), master3.seed(NETWORK, PASSPHRASE).unwrap());
    }

    // https://github.com/trezor/python-mnemonic/blob/master/vectors.json
    #[test]
    fn crosscheck_with_bip39_test_vector() {
        let secp = Secp256k1::new();

        test_xprv(&secp, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", Some("TREZOR"),
                  "xprv9s21ZrQH143K3h3fDYiay8mocZ3afhfULfb5GX8kCBdno77K4HiA15Tg23wpbeF1pLfs1c5SPmYHrEpTuuRhxMwvKDwqdKiGJS9XFKzUsAF");

        test_xprv(&secp, "legal winner thank year wave sausage worth useful legal winner thank yellow", Some("TREZOR"),
        "xprv9s21ZrQH143K2gA81bYFHqU68xz1cX2APaSq5tt6MFSLeXnCKV1RVUJt9FWNTbrrryem4ZckN8k4Ls1H6nwdvDTvnV7zEXs2HgPezuVccsq");

        test_xprv(&secp, "letter advice cage absurd amount doctor acoustic avoid letter advice cage above", Some("TREZOR"),
                  "xprv9s21ZrQH143K2shfP28KM3nr5Ap1SXjz8gc2rAqqMEynmjt6o1qboCDpxckqXavCwdnYds6yBHZGKHv7ef2eTXy461PXUjBFQg6PrwY4Gzq");

        test_xprv(&secp, "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong", Some("TREZOR"),
                  "xprv9s21ZrQH143K2V4oox4M8Zmhi2Fjx5XK4Lf7GKRvPSgydU3mjZuKGCTg7UPiBUD7ydVPvSLtg9hjp7MQTYsW67rZHAXeccqYqrsx8LcXnyd");
    }

    fn test_xprv<C: secp256k1::Signing + secp256k1::Verification>(secp: &Secp256k1<C>, words: &str, pd_pass: Option<&str>, xprv: &str) {

        let mnemonic = Mnemonic::from_str(words).unwrap();
        let master = MasterAccount::from_mnemonic(&mnemonic, 0, Network::Bitcoin, PASSPHRASE, pd_pass).unwrap();
        let seed = master.seed(Network::Bitcoin, PASSPHRASE).unwrap();
        let sk = ExtendedPrivKey::new_master(Network::Bitcoin, &seed.0.as_slice()).unwrap();

        let path = DerivationPath::from_str("m").unwrap();
        assert_eq!(&sk.derive_priv(&secp, &path).unwrap().to_string()[..], xprv)
    }
}