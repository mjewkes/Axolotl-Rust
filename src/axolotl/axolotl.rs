
use std::option::{Option};
use super::dh::{DH,DHKeyPair,DHShared,DHPublic};

pub trait Axolotl {
	type IdentityKey : DH;
	type RatchetKey : DH;

	type RootKey : Clone;
	type ChainKey : Clone;
	type MessageKey : Clone;

	type PlainText;
	type CipherText;


	fn kdf_initial(ab0 : &DHShared<Self::IdentityKey>, a0b : &DHShared<Self::IdentityKey>, a0b0 : &DHShared<Self::IdentityKey>) -> (Self::RootKey, Self::ChainKey);

	// This is the DH future secrecy ratchet/
	fn kdf_ratchet(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey);

	//This is the SCIMP style forward secrecy chain key iteration.
	fn kdf_message(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey);

	fn encode_message(
		message_key : &Self::MessageKey, 
		identity_key_local : &DHPublic<Self::IdentityKey>,
		identity_key_remote : &DHPublic<Self::IdentityKey>, 
		plaintext : &Self::PlainText) 
		-> Self::CipherText;

	fn decode_message(
		message_key : &Self::MessageKey,
		identity_key_local : &DHPublic<Self::IdentityKey>,
		identity_key_remote : &DHPublic<Self::IdentityKey>, 
		cyphertext : &Self::CipherText) 
		-> Option<Self::PlainText>;

	fn ratchet_keys_are_equal(key0 : &DHPublic<Self::RatchetKey>, key1 : &DHPublic<Self::RatchetKey>) -> bool;
	fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey>;

	fn future_message_limit() -> u32;
	fn chain_message_limit() -> u32;

	fn skipped_chain_limit() -> usize;
}