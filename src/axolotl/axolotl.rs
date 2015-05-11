
use std::option::{Option};
use super::dh::{DH,DHKeyPair};

pub trait Axolotl {
	type IdentityKey : DH;
	type RatchetKey : DH;

	type RootKey : Clone;
	type ChainKey : Clone;
	type MessageKey : Clone;

	type PlainText;
	type CipherText;


	fn kdf_initial(ab0 : &<Self::IdentityKey as DH>::Shared, a0b : &<Self::IdentityKey as DH>::Shared, a0b0 : &<Self::IdentityKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey);

	// This is the DH future secrecy ratchet/
	fn kdf_ratchet(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey);

	//This is the SCIMP style forward secrecy chain key iteration.
	fn kdf_message(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey);

	fn encode_message(
		message_key : &Self::MessageKey, 
		identity_key_local : &<Self::IdentityKey as DH>::Public,
		identity_key_remote : &<Self::IdentityKey as DH>::Public, 
		plaintext : &Self::PlainText) 
		-> Self::CipherText;

	fn decode_message(
		message_key : &Self::MessageKey,
		identity_key_local : &<Self::IdentityKey as DH>::Public,
		identity_key_remote : &<Self::IdentityKey as DH>::Public, 
		cyphertext : &Self::CipherText) 
		-> Option<Self::PlainText>;

	fn ratchet_keys_are_equal(key0 : &<Self::RatchetKey as DH>::Public, key1 : &<Self::RatchetKey as DH>::Public) -> bool;
	fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey>;

	fn future_message_limit() -> u32;
	fn chain_message_limit() -> u32;

	fn skipped_chain_limit() -> usize;
}