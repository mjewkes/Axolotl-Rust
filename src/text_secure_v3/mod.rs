use ::axolotl;
use ::axolotl::{AxolotlMessage,DH,DHKeyPair,DHPublic,DHShared};
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

pub struct CipherTextAndVersion{
    cipher_text : Box<[u8]>,
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

    type Mac = hmac::MacResult;

     fn derive_initial_root_key_and_chain_key(
        local_identity_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_identity_dh_secred : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>) -> (Self::RootKey, Self::ChainKey){
        
        unimplemented!();
    }

    fn derive_next_root_key_and_chain_key(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
        unimplemented!();
    }

    fn derive_next_chain_and_message_key(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){
        unimplemented!();
    }
    
    fn encrypt_message(message_key : &Self::MessageKey, 
                      plaintext : &Self::PlainText) 
                      -> Self::CipherText{

        let PlainText(ref text) = *plaintext;
        let ciphertext = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);
        
        CipherTextAndVersion {
            version : 3,
            cipher_text : ciphertext.into_boxed_slice(),
        }
    }

    fn decrypt_message(message_key : &Self::MessageKey, 
                      ciphertext : &Self::CipherText) 
                      -> Option<Self::PlainText>{
        if ciphertext.version != 3 {
            return None;
        }

        let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext.cipher_text, message_key.cipher_key, message_key.iv);
        Some(PlainText(result.into_boxed_slice()))
    }

    fn authenticate_message(
        message : &AxolotlMessage<Self>, 
        message_key : &Self::MessageKey, 
        sender_identity : &DHPublic<Self::IdentityKey>, 
        receiver_identity : &DHPublic<Self::IdentityKey>) -> Self::Mac{

        let mut mac_state = hmac::HmacSha256::new(&message_key.mac_key);
        mac_state.input(&sender_identity[..]);
        mac_state.input(&receiver_identity[..]);
        mac_state.input(&message.ciphertext.cipher_text[..]); //TODO: input the version
        hmac::truncate_mac_result(mac_state.result(), 8)
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