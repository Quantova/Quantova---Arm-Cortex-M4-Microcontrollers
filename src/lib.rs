extern crate libc;
use libc::c_uchar;

#[link(name = "pqm4", kind = "static")]
extern "C" {
    pub fn pqm4_generate_keypair(pk: *mut c_uchar, sk: *mut c_uchar);
    pub fn pqm4_encrypt(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar);
    pub fn pqm4_decrypt(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar);
}

pub struct PQM4Kyber768;

impl PQM4Kyber768 {
    // Adjust sizes based on the PQM4 API definitions
    pub fn generate_keypair() -> ([u8; 1184], [u8; 2400]) {
        let mut pk = [0u8; 1184];
        let mut sk = [0u8; 2400];
        unsafe {
            pqm4_generate_keypair(pk.as_mut_ptr(), sk.as_mut_ptr());
        }
        (pk, sk)
    }

    pub fn encrypt(pk: &[u8]) -> ([u8; 1088], [u8; 32]) {
        let mut ct = [0u8; 1088];
        let mut ss = [0u8; 32];
        unsafe {
            pqm4_encrypt(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr());
        }
        (ct, ss)
    }

    pub fn decrypt(ct: &[u8], sk: &[u8]) -> [u8; 32] {
        let mut ss = [0u8; 32];
        unsafe {
            pqm4_decrypt(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr());
        }
        ss
    }
}
