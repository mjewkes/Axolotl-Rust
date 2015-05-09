use ::axolotl;
use ::axolotl::DH;
use ::axolotl::DHKeyPair;

pub struct TextSecureV3;

pub struct IdentityKey;
impl axolotl::DH for IdentityKey {
	type Private = [u8;32];
	type Public = [u8;32];
	type Shared = [u8;32];

	fn public(key : &Self::Private) -> Self::Public{
		unimplemented!();
	}
	fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
		unimplemented!();
	}

}

pub struct RatchetKey;
impl axolotl::DH for RatchetKey {
	type Private = [u8;32];
	type Public = [u8;32];
	type Shared = [u8;32];

	fn public(key : &Self::Private) -> Self::Public{
		unimplemented!();
	}
	fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
		unimplemented!();
	}

}

#[derive(Clone)]
pub struct Rootkey ([u8;32]);

#[derive(Clone)]
pub struct ChainKey ([u8;32]);

#[derive(Clone)]
pub struct MessageKey ([u8;32]);

pub struct PlainText(Box<[u8]>);

pub struct CipherText(Box<[u8]>);



impl axolotl::Axolotl for TextSecureV3{
	type IdentityKey = IdentityKey;
	type RatchetKey = RatchetKey;

	type RootKey = Rootkey;
	type ChainKey = ChainKey;
	type MessageKey = MessageKey;

	type PlainText = PlainText;
	type CipherText = CipherText;


	fn kdf_initial(ab0 : &<Self::IdentityKey as DH>::Shared, a0b : &<Self::IdentityKey as DH>::Shared, a0b0 : &<Self::IdentityKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
		unimplemented!();
	}

	fn kdf_ratchet(root_key : Self::RootKey, ratchet : <Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
		unimplemented!();
	}

	fn kdf_message(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){
		unimplemented!();
	}

	fn encode_message(message_key : &Self::MessageKey, identity_key_local : &<Self::IdentityKey as DH>::Private, plaintext : &Self::PlainText) -> Self::CipherText{
		unimplemented!();
	}
	fn decode_message(message_key : &Self::MessageKey, identity_key_remote : &<Self::IdentityKey as DH>::Public, cyphertext : &Self::CipherText) -> Option<Self::PlainText>{
		unimplemented!();
	}

	fn ratchet_keys_are_equal(key0 : &<Self::RatchetKey as DH>::Public, key1 : &<Self::RatchetKey as DH>::Public) -> bool{
		unimplemented!();
	}
	fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey>{
		unimplemented!();
	}

	fn future_message_limit() -> u32{
		unimplemented!();
	}
	fn chain_message_limit() -> u32
	{
		unimplemented!();
	}

	fn skipped_chain_limit() -> usize{
		unimplemented!();
	}
}