//#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

use bitcoin::Network;
use bitcoin_wallet::account::{AccountAddressType, MasterAccount, MasterKeyEntropy};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jint, jstring};

use crate::account::{account_to_json, get_account, master_from_json, master_to_json};

mod account;

// org.rustwallet.android.AccountLib.getMaster(int entropy, int network, String passphrase): String;
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_AccountLib_getMaster(env: JNIEnv, _: JObject,
                                                                      j_entropy: jint,
                                                                      j_network: jint,
                                                                      j_passphrase: JString) -> jstring {
    let entropy: MasterKeyEntropy = match j_entropy as u32 {
        16 => MasterKeyEntropy::Low,
        32 => MasterKeyEntropy::Recommended,
        64 => MasterKeyEntropy::Paranoid,
        _ => MasterKeyEntropy::Recommended
    };

    let network = j_network as u32;
    let network = Network::from_magic(network).unwrap();

    let passphrase = CString::from(
        CStr::from_ptr(
            env.get_string(j_passphrase).unwrap().as_ptr()
        )
    );

    // passphrase is used to encrypt the seed in memory and in storage
    let passphrase: &str = passphrase.to_str().unwrap();

    // create new master account
    let master = MasterAccount::new(entropy, network, passphrase).unwrap();

    // JString result
    let master_json = master_to_json(&master);
    let j_master_json = env.new_string(master_json.to_owned()).unwrap();
    j_master_json.into_inner()
}

// org.rustwallet.android.AccountLib.getAccount(String masterJson, String passphrase,
//                      int accountNumber, int subAccountNumber, int seen, int lookahead): String;
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_AccountLib_getAccount(env: JNIEnv, _: JObject,
                                                                       j_master_json: JString,
                                                                       j_passphrase: JString,
                                                                       j_address_type: jint,
                                                                       j_account_number: jint,
                                                                       j_sub_account_number: jint,
                                                                       j_seen: jint,
                                                                       j_lookahead: jint) -> jstring {
    let master_json = CString::from(
        CStr::from_ptr(
            env.get_string(j_master_json).unwrap().as_ptr()
        )
    );

    // master account backup json
    let master_json: &str = master_json.to_str().unwrap();

    let passphrase = CString::from(
        CStr::from_ptr(
            env.get_string(j_passphrase).unwrap().as_ptr()
        )
    );

    // passphrase is used to encrypt the seed in memory and in storage
    let passphrase: &str = passphrase.to_str().unwrap();

    let address_type = if j_address_type > 0 { j_address_type as u32 } else { 0 as u32 };
    let address_type = AccountAddressType::from_u32(address_type);

    let account_number = if j_account_number > 0 { j_account_number as u32 } else { 0 as u32 };

    let sub_account_number = if j_sub_account_number > 0 { j_sub_account_number as u32 } else { 0 as u32 };

    let seen = if j_seen > 0 { j_seen as u32 } else { 0 as u32 };

    let lookahead = if j_lookahead > 0 { j_lookahead as u32 } else { 0 as u32 };

    let master = master_from_json(master_json);
    let account = get_account(&master, &passphrase, address_type, account_number, sub_account_number, seen, lookahead);

    // JString result
    let account_json = account_to_json(&account);
    let j_account_json = env.new_string(account_json.to_owned()).unwrap();
    j_account_json.into_inner()
}
