use std::fmt;
use std::result::{Result};

use super::key_pair::KeyPair;

pub trait Axolotl {
    type PrivateKey : Clone;
    type PublicKey : Clone;
    type SharedSecret : Clone;
    type InitialSharedSecret;
    type SessionIdentity;

    type RootKey : Clone;
    type ChainKey : Clone;
    type MessageKey : Clone;

    type PlainText;
    type CipherText;
    type Message;

    type Mac : PartialEq;

    type EncryptError : fmt::Debug;
    type EncodeError : fmt::Debug;
    type DecryptError : fmt::Debug;
    type DecodeError : fmt::Debug;

    fn derive_initial_root_key_and_chain_key(
        &self, Self::InitialSharedSecret) 
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
        plaintext : Self::PlainText) 
    -> Result<Self::CipherText, Self::EncryptError>;

    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey,
        cyphertext : Self::CipherText) 
    -> Result<Self::PlainText,Self::DecryptError>;

    fn authenticate_message(
        &self,
        message : &Self::Message, 
        message_key : &Self::MessageKey,
        identity : &Self::SessionIdentity)
    -> Self::Mac;

    fn encode_header_and_ciphertext(
        &self,
        header : Header<Self>,
        ciphertext : Self::CipherText
    ) -> Result<Self::Message, Self::EncodeError>;

    fn decode_header(&self, message : &Self::Message
    ) -> Result<Header<Self>,Self::DecodeError>;

    fn decode_ciphertext(&self, message : Self::Message
    ) -> Result<Self::CipherText,Self::DecodeError>;

    fn ratchet_keys_are_equal(
        &self,
        key0 : &Self::PublicKey, 
        key1 : &Self::PublicKey) 
    -> bool;

    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>;

    fn derive_shared_secret(&self, key : &Self::PrivateKey, public : &Self::PublicKey) -> Self::SharedSecret;

    fn future_message_limit(&self) -> usize;

    fn chain_message_limit(&self) -> usize;

    fn skipped_chain_limit(&self) -> usize;
}

pub enum ReceiveError<T> where T:Axolotl {
    DecryptError(T::DecryptError),
    DecodeError(T::DecodeError),
    InvalidMac,
    MessageNumberTooFarAhead(usize),
    MessageNumberTooLarge(usize),
    MessageNumberAheadOfChainLength(usize),
    MessageNumberAlreadyUsed(usize),
}

impl<T> fmt::Debug for ReceiveError<T> where T:Axolotl {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ReceiveError::DecryptError(ref err) => write!(f, "DecryptError({:?})", err),
            &ReceiveError::DecodeError(ref err) => write!(f, "DecodeError({:?})", err),
            &ReceiveError::InvalidMac => write!(f, "InvalidMac"),
            &ReceiveError::MessageNumberTooFarAhead(message_number) => write!(f, "MessageNumberTooFarAhead({:?})", message_number),
            &ReceiveError::MessageNumberTooLarge(message_number) => write!(f, "MessageNumberTooLarge({:?})", message_number),
            &ReceiveError::MessageNumberAheadOfChainLength(message_number) => write!(f, "MessageNumberAheadOfChainLength({:?})", message_number),
            &ReceiveError::MessageNumberAlreadyUsed(message_number) => write!(f, "MessageNumberAlreadyUsed({:?})", message_number),
        }
    }
}

pub enum SendError<T> where T:Axolotl {
    EncryptError(T::EncryptError),
    EncodeError(T::EncodeError),
}

impl<T> fmt::Debug for SendError<T> where T:Axolotl {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SendError::EncryptError(ref err) => write!(f, "EncryptError({:?})", err),
            &SendError::EncodeError(ref err) => write!(f, "EncodeError({:?})", err),
        }
    }
}

pub struct Header<T> where T:Axolotl {
    pub message_number : usize,
    pub message_number_prev : usize,
    pub ratchet_key : T::PublicKey,
}

impl <T:Axolotl> Clone for KeyPair<T> {
    fn clone(&self) -> Self {
        KeyPair { 
            key : self.key.clone(),
            public : self.public.clone(),
        }
    }
}