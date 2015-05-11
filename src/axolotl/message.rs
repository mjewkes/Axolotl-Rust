use super::axolotl::{Axolotl};
use super::dh::{DH};

pub struct AxolotlMessage<T> where T:Axolotl {
	pub message_number : u32,
	pub ratchet_key : <T::RatchetKey as DH>::Public,
	pub ciphertext : T::CipherText,
}