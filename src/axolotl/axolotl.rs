use std::fmt;
use std::result::{Result};

use rustc_serialize::{Encodable,Decodable};

use super::key_pair::KeyPair;

/// An implementation of the [Axolotl protocol](https://github.com/trevp/axolotl/wiki).
pub trait Axolotl {
    /// The diffie hellman or elliptic curve diffie hellman private key.
    type PrivateKey : Clone + Decodable + Encodable;
    /// The diffie hellman or elliptic curve diffie hellman public key.
    type PublicKey : Clone + Decodable + Encodable;
    /// The diffie hellman or elliptic curve diffie hellman shared secret.
    type SharedSecret : Clone;
    /// This shared key used to initialize a session. It should be derived using triple diffie hellman, or a signed key exchange, or similar protocol.
    type InitialSharedSecret;
    /// This is a unique-per-session value used to salt the MAC. For example, it could be the two public keys used in the handshake.
    type SessionIdentity : Decodable + Encodable;

    type RootKey : Clone + Decodable + Encodable;
    type ChainKey : Clone + Decodable + Encodable;
    type MessageKey : Clone + Decodable + Encodable;

    type PlainText;
    type CipherText;
    /// A header together with a ciphertext. It will be authenticated and sent along with the MAC to the reveiver.
    type Message;

    type Mac : PartialEq;

    type EncryptError : fmt::Debug;
    type EncodeError : fmt::Debug;
    type DecryptError : fmt::Debug;
    type DecodeError : fmt::Debug;

    /// Derives a root key and a chain key from a shared secret.
    ///
    /// #Arguments
    ///
    /// * 'initial_secret' - A shared secret for the session, computed using triple diffie hellman, or some other protocol
    ///
    /// #Return Value
    /// 
    /// * A root key and a chain key derived from the initial secret using HKDF or a similar function
    fn derive_initial_root_key_and_chain_key(
        &self, 
        initial_secret : Self::InitialSharedSecret
    ) -> (Self::RootKey, Self::ChainKey);

    /// Derives a new root key and chain key from the previous root key, and new key material.
    ///
    /// #Arguments
    ///
    /// * 'root_key' - The previous root key
    /// * 'new_secret' - New key material from a diffie hellman key exchange
    /// 
    /// #Return Value
    ///
    /// * A root key and a chain key derived from the previous root key and new key material using HKDF or similar function
    fn derive_next_root_key_and_chain_key(
        &self,
        root_key : Self::RootKey, 
        new_secret : &Self::SharedSecret) 
    -> (Self::RootKey, Self::ChainKey);

    /// Derives a new chain key and a message key from the previous chain key.
    ///
    /// #Arguments
    ///
    /// * 'chain_key' - The current chain key
    ///
    /// #Return Value
    ///
    /// * A new chain key and message key derived by hashing the current chain key or by using a KDF
    fn derive_next_chain_and_message_key(
        &self,
        chain_key : &Self::ChainKey) 
    -> (Self::ChainKey, Self::MessageKey);

    /// Encrypts a plaintext message.
    ///
    /// #Arguments
    ///
    /// * 'message_key' - The key for the cipher
    /// * 'plaintext' - The text to encrypt
    ///
    /// #Return Value
    ///
    /// * Either the ciphertext, or an error
    fn encrypt_message(
        &self,
        message_key : &Self::MessageKey,
        plaintext : Self::PlainText) 
    -> Result<Self::CipherText, Self::EncryptError>;

    /// Decrypts a ciphertext.
    ///
    /// #Arguments
    ///
    /// * 'message_key' - The key for the cipher
    /// * 'ciphertext' - The encrypted text
    ///
    /// #Return Value
    ///
    /// * Either the plaintext, or an error
    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey,
        cyphertext : Self::CipherText) 
    -> Result<Self::PlainText,Self::DecryptError>;

    /// Computes a message authentication code for the message.
    ///
    /// #Arguments
    ///
    /// * 'message' - The message, including ciphertext and header
    /// * 'message_key' - The key for the MAC function
    /// * 'identity' - A unique-per-session value to use as a salt
    ///
    /// #Return Value
    ///
    /// * A message authentication code used to check for errors or tampering
    fn authenticate_message(
        &self,
        message : &Self::Message, 
        message_key : &Self::MessageKey,
        identity : &Self::SessionIdentity)
    -> Self::Mac;

    /// Encodes a message from a ciphertext and message header.
    ///
    /// #Arguments
    ///
    /// * 'header' - The message header: mesage number, previous message number, and ratchet key
    /// * 'ciphertext' - The ciphertext
    ///
    /// #Return Value
    ///
    /// * Either an encoded message to be authenticated, or an error
    fn encode_header_and_ciphertext(
        &self,
        header : Header<Self>,
        ciphertext : Self::CipherText
    ) -> Result<Self::Message, Self::EncodeError>;

    /// Extracts the header from the message.
    ///
    /// #Arguments
    ///
    /// * 'message' - The message to extract the header from
    ///
    /// #Return Value
    ///
    /// * Either the header, or an error
    fn decode_header(&self, message : &Self::Message
    ) -> Result<Header<Self>,Self::DecodeError>;

    /// Extracts the ciphertext from the message and consumes the message.
    ///
    /// #Arguments
    ///
    /// * 'message' - The message to get the ciphertext from
    ///
    /// #Return Value
    ///
    /// * Either the ciphertext, or an error
    fn decode_ciphertext(&self, message : Self::Message
    ) -> Result<Self::CipherText,Self::DecodeError>;

    /// Checks that two ratchet keys are equal.
    ///
    /// #Arguments
    ///
    /// * 'key0' - A public key
    /// * 'key1' - Another public key
    ///
    /// #Return Value
    ///
    /// * True iff key0 and key1 are equal
    fn ratchet_keys_are_equal(
        &self,
        key0 : &Self::PublicKey, 
        key1 : &Self::PublicKey) 
    -> bool;

    /// Generates a new key pair.
    ///
    /// #Return Value
    ///
    /// * A random new key and its corresponding public key
    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>;

    /// Derives a shared secret from your public key and their public key.
    ///
    /// #Arguments
    ///
    /// * 'key' - Your private key
    /// * 'public' - Their public key
    ///
    /// #Return Value
    ///
    /// * A shared secret. Should be the same value as derive_shared_secret(their_private_key, your_public_key) so that both parties have the same secret.
    fn derive_shared_secret(&self, key : &Self::PrivateKey, public : &Self::PublicKey) -> Self::SharedSecret;

    /// Specifies the number of messages ahead of the last received one to consider.
    ///
    /// #Return Value
    ///
    /// * The number of messages to allow ahead in the current chain
    fn future_message_limit(&self) -> usize;

    /// Specifies the number of messages allowed per chain.
    ///
    /// #Return Value
    ///
    /// * The number of messages allowed in each chain
    fn chain_message_limit(&self) -> usize;

    /// Specifies the number of message chains to keep track of at once.
    ///
    /// #Return Value
    ///
    /// * The number of chains to keep for decrypting delayed messages
    fn skipped_chain_limit(&self) -> usize;
}

/// Represents errors that can occur when attempting to decrypt a message.
pub enum ReceiveError<T> where T:Axolotl {
    /// This error means decrypt_message failed.
    DecryptError(T::DecryptError),
    /// This error means decode_header or decode_ciphertext failed.
    DecodeError(T::DecodeError),
    /// This error means that the computed MAC didn't match the received MAC.
    InvalidMac,
    /// This error means that a message was received with a message number too far ahead of the current chain.
    MessageNumberTooFarAhead(usize),
    /// This error means that a message was received with a message number above the chain limit.
    MessageNumberTooLarge(usize),
    /// This error means that a message was received with a message number out of the bounds of its chain.
    MessageNumberAheadOfChainLength(usize),
    /// This error means that a message was received with a duplicate message number.
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

/// Represents errors that can occur when encrypting a message.
pub enum SendError<T> where T:Axolotl {
    /// This error means encrypt_message failed.
    EncryptError(T::EncryptError),
    /// This error means encode_header_and_ciphertext failed.
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

/// An Axolotl message header.
pub struct Header<T> where T:Axolotl {
    /// This is the number of messages already sent using the same chain.
    pub message_number : usize,
    /// This is the number of message sent using the previous chain.
    pub message_number_prev : usize,
    /// This is the ratchet key for the current chain.
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
