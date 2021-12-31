use aes_gcm::aead::heapless::Vec;
use aes_gcm::aead::{AeadInPlace, NewAead};
use aes_gcm::aes::cipher::generic_array::typenum::bit::{B0, B1};
use aes_gcm::aes::cipher::generic_array::typenum::{UInt, UTerm};
use aes_gcm::aes::cipher::generic_array::GenericArray;
use aes_gcm::aes::Aes256;
use aes_gcm::{Aes256Gcm, AesGcm, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;
use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use std::iter::repeat;

type KeyType = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

type NonceType = GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

type CipherType = AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

pub type Base64String = String;
pub struct Crypt {
    key: KeyType,
    nonce: NonceType,
    cipher: CipherType,
}
impl Crypt {
    pub fn new_random(master_passwd: &str) -> Self {
        let len = master_passwd.len();
        let mut passwd = master_passwd.as_bytes().to_vec();

        for _i in 0..32 - len {
            passwd.push(0u8);
        }
        let key = GenericArray::from_slice(&passwd);

        let mut nonce_raw: Vec<u8, 12> = repeat(0u8).take(12).collect();
        OsRng.fill_bytes(&mut nonce_raw[..]);

        Crypt {
            key: *key,
            nonce: Nonce::from_slice(&nonce_raw).clone(),
            cipher: Aes256Gcm::new(&key),
        }
    }
    pub fn from_base64_key_nounce(master_passwd: &str, nonce: &str) -> Self {
        let len = master_passwd.len();
        let mut passwd = master_passwd.as_bytes().to_vec();

        for _i in 0..32 - len {
            passwd.push(0u8);
        }
        let key = GenericArray::from_slice(&passwd);

        Crypt {
            key: *key,
            nonce: Nonce::from_slice(&nonce.from_base64().unwrap()).clone(),
            cipher: Aes256Gcm::new(&key),
        }
    }
    pub fn get_key_base64(&self) -> String {
        self.key.to_vec().to_base64(STANDARD)
    }
    pub fn get_nonce_base64(&self) -> String {
        self.nonce.to_vec().to_base64(STANDARD)
    }

    pub fn encrypt(&mut self, passwd: &str) -> String {
        let mut buffer: Vec<u8, 128> = Vec::new(); // Buffer needs 16-bytes overhead for GCM tag
        buffer.extend_from_slice(passwd.as_bytes());

        self.cipher
            .encrypt_in_place(&self.nonce, &self.key, &mut buffer)
            .expect("encryption failure!");

        buffer.to_base64(STANDARD)
    }
    pub fn decrypt(&mut self, encrypted_passwd: &str) -> Base64String {
        let mut buffer = encrypted_passwd.from_base64().expect("No valid Base64");
        self.cipher
            .decrypt_in_place(&self.nonce, &self.key, &mut buffer)
            .expect("decryption failure!");

        String::from_utf8(buffer).expect("No valid UTF-8")
    }
}
