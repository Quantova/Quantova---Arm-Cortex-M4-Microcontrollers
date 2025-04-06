mod lib;

fn main() {
    println!("Testing PQM4 Kyber768 integration...");

    let (pk, sk) = lib::PQM4Kyber768::generate_keypair();
    println!("Keypair generated.");

    let (ct, ss_enc) = lib::PQM4Kyber768::encrypt(&pk);
    println!("Encryption complete.");

    let ss_dec = lib::PQM4Kyber768::decrypt(&ct, &sk);
    println!("Decryption complete.");

    if ss_enc == ss_dec {
        println!("Success: Shared secrets match.");
    } else {
        println!("Error: Shared secrets do not match.");
    }
}
