use std::option::{Option};

pub trait Axolotl {
    type PrivateKey : Clone;
    type PublicKey : Clone;
    type SharedSecret : Clone;

    type RootKey : Clone;
    type ChainKey : Clone;
    type MessageKey : Clone;

    type PlainText;
    type CipherText;

    type Mac : PartialEq;
    type Message;


    fn derive_initial_root_key_and_chain_key(
        &self,
        local_identity_remote_handshake_dh_secret : &Self::SharedSecret, 
        local_handshake_remote_identity_dh_secred : &Self::SharedSecret, 
        local_handshake_remote_handshake_dh_secret : &Self::SharedSecret) 
    -> (Self::RootKey, Self::ChainKey);

    // This is the DH future secrecy ratchet/
    fn derive_next_root_key_and_chain_key(
        &self,
        root_key : Self::RootKey, 
        ratchet : &Self::SharedSecret) 
    -> (Self::RootKey, Self::ChainKey);

    //This is the SCIMP style forward secrecy chain key iteration.
    fn derive_next_chain_and_message_key(
        &self,
        chain_key : &Self::ChainKey) 
    -> (Self::ChainKey, Self::MessageKey);

    fn encrypt_message(
        &self,
        message_key : &Self::MessageKey,
        plaintext : &Self::PlainText) 
    -> Self::CipherText;

    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey,
        cyphertext : &Self::CipherText) 
    -> Option<Self::PlainText>;

    fn authenticate_message(
        &self,
        message : &Self::Message,
        message_key : &Self::MessageKey, 
        sender_identity : &Self::PublicKey, 
        receiver_identity : &Self::PublicKey)
    -> Self::Mac;

    fn ratchet_keys_are_equal(
        &self,
        key0 : &Self::PublicKey, 
        key1 : &Self::PublicKey) 
    -> bool;

    fn encode_header_and_ciphertext(
        &self,
        header : (usize, Self::PublicKey),
        ciphertext : Self::CipherText
    ) -> Self::Message;

    fn with_decoded_header<F,T>(&self, message : &Self::Message, f : F
    ) -> T where F:FnOnce(usize, &Self::PublicKey) -> T;

    fn with_decoded_ciphertext<F,T>(&self, message : &Self::Message, f : F
    ) -> T where F:FnOnce(&Self::CipherText) -> T;

    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>;

    fn derive_shared_secret(&self, key : &Self::PrivateKey, public : &Self::PublicKey) -> Self::SharedSecret;

    fn derive_public_key(&self, key : &Self::PrivateKey) -> Self::PublicKey;

    fn future_message_limit(&self) -> usize;

    fn chain_message_limit(&self) -> usize;

    fn skipped_chain_limit(&self) -> usize;
}

pub struct KeyPair<T> where T:Axolotl {
    pub key : T::PrivateKey,
    pub public : T::PublicKey,
}

impl <T:Axolotl> Clone for KeyPair<T> {
    fn clone(&self) -> Self {
        KeyPair { 
            key : self.key.clone(),
            public : self.public.clone(),
        }
    }
}