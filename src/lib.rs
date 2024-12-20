#![no_std]

pub extern crate alloc;
pub extern crate zephyr;

use alloc::vec::Vec;
use zephyr::printkln;


extern "C" {
    fn cs_random(dst: *mut u8, len: usize);
}

/// Crypto safe random rust wrapper
pub fn rust_cs_random_vec(len: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(len);
    unsafe {
        buffer.set_len(len);
        cs_random(buffer.as_mut_ptr(), len);
    }
    buffer
}

#[no_mangle]
extern "C" fn rust_main() {
    printkln!("\n\n\n\n\n\n\n\n\n\nHello Rust! \n");
}


#[no_mangle]
extern "C" fn test_wallet() {
    printkln!("\n1.Generate Hardware secure random number:\n");
    printkln!("128 Bit - 256 Bit, Add checksum to 11 bit split. Supports 12, 15, 21, and 24 mnemonics. This use 128 Bit.");

    let random = rust_cs_random_vec(16);
    printkln!("\nRandom: {} \n\n", ohw_wallets::alg::hex::encode(random.clone()));

    printkln!("\n2.Use random entropy generate mnemonic:");
    let mnemonic = ohw_wallets::mnemonic::Mnemonic::from_entropy(&random).unwrap();
    printkln!("\nMnemonic: {} \n\n", mnemonic.words.join(" ").as_str());

    printkln!("\n3.Mnemonic to seed. Supports mnemonic passwords, here the password is ohw.\n");
    printkln!("Key: {} \n\n", ohw_wallets::alg::hex::encode(mnemonic.clone().to_seed("ohw").unwrap()));

    printkln!("\n4.BIP32 Root Key:\n");
    let root = ohw_wallets::wallets::ExtendedPrivKey::derive(&mnemonic.to_seed("ohw").unwrap(), "m".parse().unwrap()).unwrap();
    printkln!("Key: {} \n\n", root.encode(false).unwrap());

    printkln!("\n5.BIP44 ETH Extended Private Key, m/44'/60'/0'/0 Derivation Path:\n");
    let root = ohw_wallets::wallets::ExtendedPrivKey::derive(&mnemonic.to_seed("ohw").unwrap(), "m/44'/60'/0'/0".parse().unwrap()).unwrap();
    printkln!("Key: {} \n\n", root.encode(false).unwrap());

    printkln!("\n6.ETH Account 0, m/44'/60'/0'/0/0 Derivation Path:\n");
    let root = ohw_wallets::wallets::ExtendedPrivKey::derive(&mnemonic.to_seed("ohw").unwrap(), "m/44'/60'/0'/0/0".parse().unwrap()).unwrap();
    printkln!("Key: {} \n\n", ohw_wallets::alg::hex::encode(root.secret_key));

}
