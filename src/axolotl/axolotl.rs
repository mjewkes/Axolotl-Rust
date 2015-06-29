use std::fmt;
use std::result::{Result};

use rustc_serialize::{Encodable,Decodable};

use super::key_pair::KeyPair;

pub trait Axolotl {
    /// this is the type of the diffie hellman or elliptic curve diffie hellman private key.
    type PrivateKey : Clone + Decodable + Encodable;
    /// this is the type of the diffie hellman or elliptic curve diffie hellman public key.
    type PublicKey : Clone + Decodable + Encodable;
    /// this is the type of the diffie hellman or elliptic curve diffie hellman shared secret.
    type SharedSecret : Clone;
    /// this is the type of the shared key used to initialize a session. it should be derived using triple diffie hellman, or a signed key exchange, or similar protocol.
    type InitialSharedSecret;
    /// this is a unique-per-session value used to salt the MAC. for example, it could be the two public keys used in the handshake.
    type SessionIdentity : Decodable + Encodable;

    type RootKey : Clone + Decodable + Encodable;
    type ChainKey : Clone + Decodable + Encodable;
    type MessageKey : Clone + Decodable + Encodable;

    type PlainText;
    type CipherText;
    /// this type represents a header together with a ciphertext. it will be authenticated and sent along with the MAC to the reveiver.
    type Message;

    type Mac : PartialEq;

    type EncryptError : fmt::Debug;
    type EncodeError : fmt::Debug;
    type DecryptError : fmt::Debug;
    type DecodeError : fmt::Debug;

    /// this method derives a root key and a chain key from a shared secret.
    ///
    /// #Arguments
    ///
    /// * 'initial_secret' - a shared secret for the session, computed using triple diffie hellman, or some other protocol.
    ///
    /// #Return Value
    /// 
    /// * a root key and a chain key derived from the initial secret using HKDF or a similar function.
    fn derive_initial_root_key_and_chain_key(
        &self, 
        initial_secret : Self::InitialSharedSecret
    ) -> (Self::RootKey, Self::ChainKey);

    /// this method derives a new root key and chain key from the previous root key, and new key material.
    ///
    /// #Arguments
    ///
    /// * 'root_key' - the previous root key
    /// * 'new_secret' - new key material from a diffie hellman key exchange
    /// 
    /// #Return Value
    ///
    /// * a root key and a chain key derived from the previous root key and new key material using HKDF or similar function.
    fn derive_next_root_key_and_chain_key(
        &self,
        root_key : Self::RootKey, 
        new_secret : &Self::SharedSecret) 
    -> (Self::RootKey, Self::ChainKey);

    /// this method derives a new chain key and a message key from the previous chain key.
    ///
    /// #Arguments
    ///
    /// * 'chain_key' - the current chain key
    ///
    /// #Return Value
    ///
    /// * a new chain key and message key derived by hashing the current chain key or by using a KDF
    fn derive_next_chain_and_message_key(
        &self,
        chain_key : &Self::ChainKey) 
    -> (Self::ChainKey, Self::MessageKey);

    /// encrypt a plaintext message
    ///
    /// #Arguments
    ///
    /// * 'message_key' - the key for the cipher
    /// * 'plaintext' - the text to encrypt
    ///
    /// #Return Value
    ///
    /// * either the ciphertext, or an error
    fn encrypt_message(
        &self,
        message_key : &Self::MessageKey,
        plaintext : Self::PlainText) 
    -> Result<Self::CipherText, Self::EncryptError>;

    /// decrypt a ciphertext
    ///
    /// #Arguments
    ///
    /// * 'message_key' - the key for the cipher
    /// * 'ciphertext' - the encrypted text
    ///
    /// #Return Value
    ///
    /// * either the plaintext, or an error
    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey,
        cyphertext : Self::CipherText) 
    -> Result<Self::PlainText,Self::DecryptError>;

    /// this method computes a message authentication code for the message
    ///
    /// #Arguments
    ///
    /// * 'message' - the message, including ciphertext and header
    /// * 'message_key' - the key for the MAC function
    /// * 'identity' - a unique-per-session value to use as a salt
    ///
    /// #Return Value
    ///
    /// * a message authentication code used to check for errors or tampering
    fn authenticate_message(
        &self,
        message : &Self::Message, 
        message_key : &Self::MessageKey,
        identity : &Self::SessionIdentity)
    -> Self::Mac;

    /// encode a message from a ciphertext and message header
    ///
    /// #Arguments
    ///
    /// * 'header' - the message header: mesage number, previous message number, and ratchet key
    /// * 'ciphertext' - the ciphertext
    ///
    /// #Return Value
    ///
    /// * either an encoded message to be authenticated, or an error
    fn encode_header_and_ciphertext(
        &self,
        header : Header<Self>,
        ciphertext : Self::CipherText
    ) -> Result<Self::Message, Self::EncodeError>;

    /// this method extracts the header from the message
    ///
    /// #Arguments
    ///
    /// * 'message' - the message to extract the header from
    ///
    /// #Return Value
    ///
    /// * either the header, or an error
    fn decode_header(&self, message : &Self::Message
    ) -> Result<Header<Self>,Self::DecodeError>;

    /// this method extracts the ciphertext from the message and consumes the message
    ///
    /// #Arguments
    ///
    /// * 'message' - the message to get the ciphertext from
    ///
    /// #Return Value
    ///
    /// * either the ciphertext, or an error
    fn decode_ciphertext(&self, message : Self::Message
    ) -> Result<Self::CipherText,Self::DecodeError>;

    /// this method checks that two ratchet keys are equal
    ///
    /// #Arguments
    ///
    /// * 'key0' - a public key
    /// * 'key1' - another public key
    ///
    /// #Return Value
    ///
    /// * returns true iff key0 and key1 are equal
    fn ratchet_keys_are_equal(
        &self,
        key0 : &Self::PublicKey, 
        key1 : &Self::PublicKey) 
    -> bool;

    /// this method generates a new key pair
    ///
    /// #Return Value
    ///
    /// * a random new key and its corresponding public key
    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>;

    /// this method derives a shared secret from your public key and their public key
    ///
    /// #Arguments
    ///
    /// * 'key' - your private key
    /// * 'public' - their public key
    ///
    /// #Return Value
    ///
    /// * a shared secret. should be the same value as derive_shared_secret(their_private_key, your_public_key) so that both parties have the same secret.
    fn derive_shared_secret(&self, key : &Self::PrivateKey, public : &Self::PublicKey) -> Self::SharedSecret;

    /// this method returns the number of messages ahead of the last received one to consider
    ///
    /// #Return Value
    ///
    /// * the number of messages to allow ahead in the current chain
    fn future_message_limit(&self) -> usize;

    /// this method returns the number of messages allowed per chain
    ///
    /// #Return Value
    ///
    /// * the number of messages allowed in each chain
    fn chain_message_limit(&self) -> usize;

    /// number of message chains to keep track of at once
    ///
    /// #Return Value
    ///
    /// * the number of chains to keep for decrypting delayed messages
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
