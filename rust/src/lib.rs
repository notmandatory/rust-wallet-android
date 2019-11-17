//#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::str::FromStr;

use bitcoin::Network;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin_wallet::account::{AccountAddressType, MasterAccount, MasterKeyEntropy};
use bitcoin_wallet::mnemonic::Mnemonic;
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jobject, jobjectArray, jstring};

use crate::account::{get_account, new_mnemonic};

mod account;

// org.rustwallet.android.AccountLib.getMaster(String mnemonic, long birth, int network, String passphrase, String pdPassphrase): String;
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_AccountLib_getMaster(env: JNIEnv, _: JObject,
                                                                      j_mnemonic: JString,
                                                                      j_birth: jlong,
                                                                      j_network: jint,
                                                                      j_passphrase: JString,
                                                                      j_pd_passphrase: JString) -> jobject {


    let mnemonic = env.get_string(j_mnemonic)
        .expect("error get_string j_mnemonic");
    let mnemonic = mnemonic.to_str()
        .expect("error to_str mnemonic");
    let mnemonic = Mnemonic::from_str(mnemonic)
        .expect("error Mnemonic::from_str(mnemonic)");

    let birth = u64::try_from(j_birth/1000)
        .expect("error u64::try_from(j_birth/1000)");

    let network = network_from_jint(j_network);

    let passphrase = env.get_string(j_passphrase)
        .expect("error get_string j_passphrase");
    let passphrase = passphrase.to_str()
        .expect("error to_str passphrase");

    let pd_passphrase = env.get_string(j_pd_passphrase).ok();
    let pd_passphrase = pd_passphrase.iter()
        .map(|pd| pd.to_str()
            .expect("error pd_passphrase.to_str()"))
        .next();

    // create new master account from mnemonic

    let master = MasterAccount::from_mnemonic(&mnemonic, birth, network, passphrase, pd_passphrase)
        .expect("error MasterAccount::from_mnemonic");

    let master_public = master.master_public();
    let master_public_string = master_public.to_string();
    let master_public_JString = env.new_string(&master_public_string)
        .expect("error new_string master_public_string");

    let encrypted_array: jbyteArray = env.byte_array_from_slice(&master.encrypted().to_owned())
        .expect("error byte_array_from_slice encrypted");

    let j_birth: jlong = jlong::try_from(master.birth()*1000)
        .expect("error jlong::try_from(master.birth()*1000)");

    // MasterAccount(String masterPublic, byte[] encrypted, long birth)
    let j_master = env.new_object(
        "org/rustwallet/android/MasterAccount",
        "(Ljava/lang/String;[BJ)V",
        &[JValue::Object(master_public_JString.into()),
            JValue::Object(encrypted_array.into()),
            j_birth.into()],
    ).expect("error new_object MasterAccount");

    j_master.into_inner()
}

// org.rustwallet.android.AccountLib.getAccount(MasterAccount masterJson, String passphrase,
//                      int accountNumber, int subAccountNumber, int seen, int lookahead): String;
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_AccountLib_getAccount(env: JNIEnv, _: JObject,
                                                                       j_master: JObject,
                                                                       j_passphrase: JString,
                                                                       j_address_type: jint,
                                                                       j_account_number: jint,
                                                                       j_sub_account_number: jint,
                                                                       j_seen: jint,
                                                                       j_lookahead: jint) -> jstring {
    let master = master_from_jobject(&env, &j_master);

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

    let account = get_account(&master, &passphrase, address_type, account_number, sub_account_number, seen, lookahead);

    // org.rustwallet.android.Account(int addressType, int accountNumber, int subAccountNumber, String[] instantiated, int next, int network)
    let j_account_address_type: JValue = jint::try_from(account.address_type().as_u32())
        .expect("error jint::try_from(account.address_type)").into();

    let j_account_account_number: JValue = jint::try_from(account.account_number())
        .expect("error jint::try_from(account.account_number)").into();

    let j_account_sub_account_number: JValue = jint::try_from(account.sub_account_number())
        .expect("error jint::try_from(account.sub_account_number)").into();

    let j_instantiated_vec: Vec<JString> = account.instantiated().iter()
        // return instantiated keys as address base58 strings
        .map(|i| i.address.to_string())
        .map(|a| env.new_string(a).expect("error env.new_string(a)"))
        .collect();

    let j_instantiated_arr: jobjectArray = env.new_object_array(i32::try_from(j_instantiated_vec.len()).unwrap(),
                                                                env.find_class("java/lang/String").expect("error env.find_class(String)"),
                                                                env.new_string("").expect("error env.new_string()").into())
        .expect("error env.new_object_array()");


    for i in 0..(j_instantiated_vec.len()) {
        env.set_object_array_element(j_instantiated_arr, i32::try_from(i).unwrap(),
                                     j_instantiated_vec[i].into()).expect("error set_object_array_element");
    }

    let j_next: JValue = jint::try_from(account.next())
        .expect("error jint::try_from(account.next)").into();

    let j_network: JValue = match account.network() {
        Network::Bitcoin => 1 as jint,
        Network::Testnet => 2 as jint,
        Network::Regtest => 3 as jint,
    }.into();

    let j_account = env.new_object(
        "org/rustwallet/android/Account",
        "(III[Ljava/lang/String;II)V",
        &[j_account_address_type, j_account_account_number, j_account_sub_account_number,
            JValue::Object(j_instantiated_arr.into()), j_next, j_network],
    ).expect("error new_object Account");

    j_account.into_inner()
}

//   org.rustwallet.android.AccountLib.getMnemonic(MasterAccount master, String passphrase) : String[];
#[no_mangle]
pub unsafe extern fn Java_org_rustwallet_android_AccountLib_getMnemonic(env: JNIEnv, _: JObject,
                                                                        j_entropy: jint) -> jstring {
    let entropy = entropy_from_jint(j_entropy);
    let mnemonic = new_mnemonic(entropy);
    let mnemonic_JString = env.new_string(&mnemonic.to_string())
        .expect("error env.new_string(mnemonic)");

    mnemonic_JString.into_inner()
}

// helpers used for JNI

fn entropy_from_jint(size: jint) -> MasterKeyEntropy {
    match size {
        16 => Some(MasterKeyEntropy::Low),
        32 => Some(MasterKeyEntropy::Recommended),
        64 => Some(MasterKeyEntropy::Paranoid),
        _ => None
    }.expect("invalid entropy size")
}

fn network_from_jint(network_enum_ordinal: jint) -> Network {
    match network_enum_ordinal {
        0 => Some(Network::Bitcoin),
        1 => Some(Network::Testnet),
        2 => Some(Network::Regtest),
        _ => None
    }.expect("invalid network enum ordinal")
}

// org.rustwallet.android.MasterAccount
//
//    java.lang.String getMasterPublic()
//    java.nio.ByteBuffer getEncrypted()
//    java.util.Date getBirth()
//
fn master_from_jobject(env: &JNIEnv, j_master: &JObject) -> MasterAccount {
    let j_master_public = env.call_method(*j_master, "getMasterPublic", "()Ljava/lang/String;", &[])
        .expect("error MasterAccount.getMasterPublic()");
    let j_master_public = j_master_public.l()
        .expect("error j_master_public JValue.l() to JObject");
    let public_master_key = env.get_string(JString::from(j_master_public))
        .expect("error env.get_string(j_master_public)");
    let public_master_key = public_master_key.to_str()
        .expect("error master_public to &str");

    let public_master_key: ExtendedPubKey = ExtendedPubKey::from_str(public_master_key)
        .expect("error serde_json::from_str(master_public)");

    let j_encrypted = env.call_method(*j_master, "getEncrypted", "()[B", &[])
        .expect("error MasterAccount.getEncrypted()");
    let j_encrypted = j_encrypted.l()
        .expect("error j_encrytped.l()");
    let j_encrypted = j_encrypted.into_inner() as jbyteArray;
    let encrypted = env.convert_byte_array(j_encrypted)
        .expect("error env.convert_type_array(j_encrypted)");

    let j_birth = env.call_method(*j_master, "getBirth", "()Ljava/util/Date;", &[])
        .expect("error MasterAccount.getBirth()");
    let j_birth = j_birth.l()
        .expect("error MasterAccount.getBirth() to jobject");
    let j_birth = env.call_method(j_birth, "getTime", "()J", &[])
        .expect("error birth.getTime()");
    let j_birth = j_birth.j()
        .expect("error birth.getTime() to jlong");
    let birth: u64 = u64::try_from(j_birth/1000)
        .expect("error u64::try_from(j_birth/1000");

    MasterAccount::from_encrypted(encrypted.as_slice(), public_master_key, birth)
}
