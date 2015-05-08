use ::axolotl;
use ::axolotl::DH;
use ::axolotl::DHKeyPair;
use ::crypto_wrappers::aes_cbc;
use ::crypto_wrappers::hmac;

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
pub struct MessageKey{
	cipher_key : [u8;32],
	mac_key : [u8;32],
	iv : [u8;16],
}

pub struct PlainText(Box<[u8]>);

pub struct CipherTextAndVersion{
	cipher_text : Box<[u8]>,
	mac : [u8;8],
	version : u8,
}

impl axolotl::Axolotl for TextSecureV3{
	type IdentityKey = IdentityKey;
	type RatchetKey = RatchetKey;

	type RootKey = Rootkey;
	type ChainKey = ChainKey;
	type MessageKey = MessageKey;

	type PlainText = PlainText;
	type CipherText = CipherTextAndVersion;


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
	fn decode_message(message_key : &Self::MessageKey, identity_key_remote : &<Self::IdentityKey as DH>::Public, ciphertext : &Self::CipherText) -> Option<Self::PlainText>{
		if ciphertext.version != 3{
			return None;
		}
		{
			//HMAC
			let ref data = ciphertext.cipher_text;
			let their_mac = ciphertext.mac;
			let our_mac = 
				hmac::hmac_sha256(&data,&message_key.mac_key);
				// Do three more passes.
				// Compare last 8 bytes.

			unimplemented!();
		}

		let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext.cipher_text, message_key.cipher_key, message_key.iv);
		match (result){
			Ok (r) => {
				Some(PlainText(r.into_boxed_slice()))
			},
			Err (e) => None
		}
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