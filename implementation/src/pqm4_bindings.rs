extern "C" {
    pub fn pqm4_kyber_keypair(public_key: *mut u8, secret_key: *mut u8);
    pub fn pqm4_kyber_encrypt(ciphertext: *mut u8, shared_secret: *mut u8, public_key: *const u8);
    pub fn pqm4_kyber_decrypt(shared_secret: *mut u8, ciphertext: *const u8, secret_key: *const u8);
}
mod pqm4_bindings;
use std::ptr;

fn main() {
    let mut public_key = [0u8; 800];
    let mut secret_key = [0u8; 1600];
    let mut ciphertext = [0u8; 900];
    let mut shared_secret_enc = [0u8; 32];
    let mut shared_secret_dec = [0u8; 32];

    println!("Generating Kyber Keypair...");
    unsafe {
        pqm4_bindings::pqm4_kyber_keypair(public_key.as_mut_ptr(), secret_key.as_mut_ptr());
        pqm4_bindings::pqm4_kyber_encrypt(
            ciphertext.as_mut_ptr(),
            shared_secret_enc.as_mut_ptr(),
            public_key.as_ptr(),
        );
        pqm4_bindings::pqm4_kyber_decrypt(
            shared_secret_dec.as_mut_ptr(),
            ciphertext.as_ptr(),
            secret_key.as_ptr(),
        );
    }

    println!("Kyber Encryption & Decryption Completed!");
}
