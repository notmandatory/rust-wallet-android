//#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::borrow::Borrow;
use std::ffi::{CStr, CString};

use bitcoin::Network;
use bitcoin_wallet::account::{Account, AccountAddressType, MasterAccount, Unlocker};
use bitcoin_wallet::mnemonic::Mnemonic;
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jint, jstring};

// org.rustwallet.android.Address.getReceiveAddress(String words, String passphrase);
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_Address_getReceiveAddress(env: JNIEnv, _: JObject,
                                                                          j_words: JString,
                                                                          j_passphrase: JString,
                                                                          j_sub_account_number: jint) -> jstring {
    let words = CString::from(
        CStr::from_ptr(
            env.get_string(j_words).unwrap().as_ptr()
        )
    );

    // mnemonic seed words
    let words: &str = words.to_str().unwrap();

    let passphrase = CString::from(
        CStr::from_ptr(
            env.get_string(j_passphrase).unwrap().as_ptr()
        )
    );

    let sub_account_number = if j_sub_account_number > 0 { j_sub_account_number as u32} else {0 as u32};

    // passphrase is used to encrypt the seed in memory and in storage
    let passphrase: &str = passphrase.to_str().unwrap();

    let receive_address = get_receive_address(&words, &passphrase, sub_account_number);

    // JString result
    let j_receive_address = env.new_string(receive_address.to_owned()).unwrap();
    j_receive_address.into_inner()
}

fn get_receive_address(words: &str, passphrase: &str, sub_account_number: u32) -> String {

    // re-create a master from a known mnemonic
    let mnemonic = Mnemonic::from_str(words).unwrap();
    let mut master = MasterAccount::from_mnemonic(&mnemonic, 0, Network::Bitcoin, &passphrase, None).unwrap();

    // create an unlocker that is able to decrypt the encrypted mnemonic and then calculate private keys
    let mut unlocker = Unlocker::new_for_master(&master, passphrase).unwrap();

    // create a P2WPKH (pay-to-witness-public-key-hash) (native single key segwit) account.
    // account number X, sub-account 0 (which usually means receiver) BIP32 look-ahead 10
    let account = Account::new(&mut unlocker, AccountAddressType::P2WPKH, 0, sub_account_number, 10).unwrap();
    master.add_account(account);

    // pay to some native segwit address
    let target = master.get_mut((sub_account_number, 0)).unwrap().next_key().unwrap().address.borrow();

    // return target Address as base58 string
    target.to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_receive_address;

    const WORDS: &str = "announce damage viable ticket engage curious yellow ten clock finish burden orient faculty rigid smile host offer affair suffer slogan mercy another switch park";
    const PASSPHRASE: &str = "correct horse battery staple";

    // get first receive address, m / 84'/0'/0'/0 / 0
    #[test]
    fn test_get_receive_address() {
        let test_receive_address = get_receive_address(WORDS, PASSPHRASE, 0);

        assert_eq!(test_receive_address, "bc1qlz2h9scgalmqj43d36f58dcxrrl7udu999gcp2");
    }
}
