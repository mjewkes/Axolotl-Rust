use super::axolotl::{Axolotl};

pub struct AxolotlMessage<T> where T:Axolotl {
    pub message_number : usize,
    pub ratchet_key : T::PublicKey,
    pub ciphertext : T::CipherText,
}