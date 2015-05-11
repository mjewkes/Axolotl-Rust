use ::axolotl;
use ::axolotl::{DH,DHKeyPair};
use ::crypto_wrappers::aes_cbc;
use ::crypto_wrappers::hmac;

pub struct TextSecureV3{
    my_identity_key : axolotl::DHPublic<IdentityKey>,
    their_identity_key : axolotl::DHPublic<IdentityKey>,
}
impl TextSecureV3{
    pub fn new(my_identity_key : axolotl::DHPublic<IdentityKey>, 
               their_identity_key : axolotl::DHPublic<IdentityKey>,) 
               -> TextSecureV3{
        TextSecureV3{my_identity_key : my_identity_key, their_identity_key: their_identity_key}
    } 
    fn match_mac_from_keys_and_bytes_to_truncated_mac (serialized_message_bytes : &[u8],
                                          mac_key : [u8;32],
                                          sender_public_key : &axolotl::DHPublic<IdentityKey>,
                                          receiver_public_key : &axolotl::DHPublic<IdentityKey>,
                                          truncated_mac : [u8;8])
                                          -> bool{
        let mut mac_context= hmac::HmacSha256::new(&mac_key);
        mac_context.input(sender_public_key);
        mac_context.input(receiver_public_key);
        mac_context.input(serialized_message_bytes);
        let mac_result = mac_context.result();
        let bytes = &mac_result.code()[0..8];
        bytes == truncated_mac
    }

}
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

pub struct CipherTextMacAndVersion{
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
	type CipherText = CipherTextMacAndVersion;


	fn kdf_initial(ab0 : &<Self::IdentityKey as DH>::Shared, a0b : &<Self::IdentityKey as DH>::Shared, a0b0 : &<Self::IdentityKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
		unimplemented!();
	}

	fn kdf_ratchet(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
		unimplemented!();
	}

	fn kdf_message(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){
		unimplemented!();
	}
	
	fn encode_message(message_key : &Self::MessageKey, 
		              plaintext : &Self::PlainText) 
	                  -> Self::CipherText{

		let PlainText(ref text) = *plaintext;
		let cipher_data_result = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);
		
		 //
		let mac = [0_u8;8];
		let cipher_text_out = CipherText{
			version=3,
			cipher_text=cipher_data_result.unwrap(),

		}
		unimplemented!();
		//
		
		let cipher_text_out = CipherTextMacAndVersion {
			version : 3,
			cipher_text : cipher_data_result.unwrap().into_boxed_slice(),
			mac : mac,
		};

	}
	// fn mac_from_keys_and_bytes (cipher_text_bytes : &[u8],
	//                             sender_public_key : &<Self::IdentityKey as DH>::Public,
	//                             receiver_public_key : &<Self::IdentityKey as DH>::Public,
	//                             )-> &[u8]{

	// }
	fn decode_message(message_key : &Self::MessageKey, 
		              ciphertext : &Self::CipherText) 
	                  -> Option<Self::PlainText>{

      	{
			if ciphertext.version != 3{
				return None;
			}
		}
		{
			//HMAC - this is not correct.
			unimplemented!();
			let ref data = ciphertext.cipher_text;
			let their_mac = ciphertext.mac;
			let mut mac_context=
				hmac::HmacSha256::new(&message_key.mac_key);
			mac_context.input(identity_key_remote);
			mac_context.input(data);

			let mac_result = mac_context.result();
			let mac_data = mac_result.code();
			let first_8_bytes = &mac_data[0..8];
			if first_8_bytes != ciphertext.mac {
				return None;
			}
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
		2000
	}

	fn skipped_chain_limit() -> usize{
		unimplemented!();
	}
}