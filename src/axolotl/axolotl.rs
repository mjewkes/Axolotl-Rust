use std::option::{Option};

use super::dh::{DH,DHKeyPair,DHShared,DHPublic};
use super::message::{AxolotlMessage};

pub trait Axolotl {
    type IdentityKey : DH;
    type RatchetKey : DH;

    type RootKey : Clone;
    type ChainKey : Clone;
    type MessageKey : Clone;

    type PlainText;
    type CipherText;

    type Mac : PartialEq;


    fn derive_initial_root_key_and_chain_key(
        local_identity_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_identity_dh_secred : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>) 
    -> (Self::RootKey, Self::ChainKey);

    // This is the DH future secrecy ratchet/
    fn derive_next_root_key_and_chain_key(
        root_key : Self::RootKey, 
        ratchet : &<Self::RatchetKey as DH>::Shared) 
    -> (Self::RootKey, Self::ChainKey);

    //This is the SCIMP style forward secrecy chain key iteration.
    fn derive_next_chain_and_message_key(
        chain_key : &Self::ChainKey) 
    -> (Self::ChainKey, Self::MessageKey);

    fn encrypt_message(
        message_key : &Self::MessageKey,
        plaintext : &Self::PlainText) 
    -> Self::CipherText;

    fn decrypt_message(
        message_key : &Self::MessageKey,
        cyphertext : &Self::CipherText) 
    -> Option<Self::PlainText>;

    fn authenticate_message(
        message : &AxolotlMessage<Self>, 
        message_key : &Self::MessageKey, 
        sender_identity : &DHPublic<Self::IdentityKey>, 
        receiver_identity : &DHPublic<Self::IdentityKey>)
    -> Self::Mac;

    fn ratchet_keys_are_equal(
        key0 : &DHPublic<Self::RatchetKey>, 
        key1 : &DHPublic<Self::RatchetKey>) 
    -> bool;

    fn generate_ratchet_key_pair() 
    -> DHKeyPair<Self::RatchetKey>;

    fn future_message_limit() -> u32;

    fn chain_message_limit() -> u32;

    fn skipped_chain_limit() -> usize;
}