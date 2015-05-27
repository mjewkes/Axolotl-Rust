use std::borrow::Borrow;
use super::axolotl::{Axolotl, AxolotlMessageRef, KeyPair};

pub struct AxolotlState<T> where T:Axolotl {
    root_key : T::RootKey,
    identity_key_local  : T::PublicKey,
    identity_key_remote : T::PublicKey,
    message_number_send : usize,

    chain_key_send : T::ChainKey,
    ratchet_key_send : KeyPair<T>,

    receive_chains : Vec<ReceiveChain<T>>,
}

struct ReceiveChain<T> where T:Axolotl {
    chain_key : T::ChainKey,
    chain_key_index : usize,
    ratchet_key : T::PublicKey,
    message_keys : Vec<(usize,T::MessageKey)>,
}



pub fn init_as_alice<T>(
    axolotl_impl : &T,
    identity_key_local : &T::PrivateKey,
    identity_key_remote : &T::PublicKey,
    handshake_key_local : &T::PrivateKey,
    handshake_key_remote : &T::PublicKey,
    initial_ratchet_key : &T::PublicKey
    ) -> AxolotlState<T> 
    where T:Axolotl {
        let ratchet_keypair = axolotl_impl.generate_ratchet_key_pair();
        init_as_alice_with_explicit_ratchet_keypair(
            axolotl_impl,
            identity_key_local,
            identity_key_remote,
            handshake_key_local,
            handshake_key_remote,
            ratchet_keypair,
            initial_ratchet_key
        ) 

}
pub fn init_as_alice_with_explicit_ratchet_keypair<T>(
    axolotl_impl : &T,
    identity_key_local : &T::PrivateKey,
    identity_key_remote : &T::PublicKey,
    handshake_key_local : &T::PrivateKey,
    handshake_key_remote : &T::PublicKey,
    my_ratchet_keypair : KeyPair<T>,
    initial_ratchet_key : &T::PublicKey) 
-> AxolotlState<T> 
    where T:Axolotl {
        let ab0 = axolotl_impl.derive_shared_secret(&identity_key_local, &handshake_key_remote);
        let a0b = axolotl_impl.derive_shared_secret(&handshake_key_local, &identity_key_remote);
        let a0b0 = axolotl_impl.derive_shared_secret(&handshake_key_local, &handshake_key_remote);
        let (pre_root_key, chain_key_recv) = axolotl_impl.derive_initial_root_key_and_chain_key(&ab0, &a0b, &a0b0);
        let ratchet_key = my_ratchet_keypair;
        let ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&ratchet_key.key, initial_ratchet_key);
        let (root_key,chain_key_send) = axolotl_impl.derive_next_root_key_and_chain_key(pre_root_key, &ratchet_key_derive_shared_secret);
        let initial_receive_chain = ReceiveChain {
            chain_key : chain_key_recv,
            chain_key_index : 0,
            ratchet_key : initial_ratchet_key.clone(),
            message_keys : Vec::new(),
        };
        AxolotlState {
            root_key : root_key,
            identity_key_local : axolotl_impl.derive_public_key(&identity_key_local),
            identity_key_remote : identity_key_remote.clone(),
            message_number_send : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : ratchet_key,
            receive_chains : vec![initial_receive_chain],
        }
}

pub fn init_as_bob<T>(
    axolotl_impl : &T,
    identity_key_local : &T::PrivateKey,
    identity_key_remote : &T::PublicKey,
    handshake_key_local : &T::PrivateKey,
    handshake_key_remote : &T::PublicKey,
    initial_ratchet_key : KeyPair<T>) 
-> AxolotlState<T> 
    where T:Axolotl {
        let ab0 = axolotl_impl.derive_shared_secret(&handshake_key_local, &identity_key_remote);
        let a0b = axolotl_impl.derive_shared_secret(&identity_key_local, &handshake_key_remote);
        let a0b0 = axolotl_impl.derive_shared_secret(&handshake_key_local, &handshake_key_remote);
        let (root_key, chain_key_send) = axolotl_impl.derive_initial_root_key_and_chain_key(&ab0, &a0b, &a0b0);
        AxolotlState {
            root_key : root_key,
            identity_key_local : axolotl_impl.derive_public_key(&identity_key_local),
            identity_key_remote : identity_key_remote.clone(),
            message_number_send : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : initial_ratchet_key,
            receive_chains : Vec::new(),
        }
}



impl <T:Axolotl> ReceiveChain<T> {
    fn try_get_message_key_index(&mut self, axolotl_impl : &T, index : usize) -> Option<usize> {
        if index > axolotl_impl.chain_message_limit() {
            return None;
        }
        //TODO: make sure this doesn't overflow
        if index > self.chain_key_index + axolotl_impl.future_message_limit() {
            return None;
        }
        if index < self.chain_key_index {
            for i in 0..self.message_keys.len() {
                let (chain_index, _) = self.message_keys[i];
                if chain_index == index {
                    return Some(i);
                }
            }
            return None;
        }

        for i in self.chain_key_index..(index+1) {
            let (next_chain_key, message_key) = axolotl_impl.derive_next_chain_and_message_key(&self.chain_key);
            self.message_keys.push((i,message_key));
            self.chain_key = next_chain_key;
            self.chain_key_index += 1;
        }

        return Some(self.message_keys.len()-1);
    }

    fn try_decrypt_with_message_key_index<'a> (
        &self,
        axolotl_impl : &T,
        message : &'a T::Message, 
        mac : T::Mac,
        sender_identity : &T::PublicKey, 
        receiver_identity : &T::PublicKey,
        message_key_index : usize,
    ) -> Option<T::PlainText> where &'a T::Message : AxolotlMessageRef<T> {
        let (_,ref message_key) = self.message_keys[message_key_index];
        let expected_mac = axolotl_impl.authenticate_message(message, message_key, sender_identity, receiver_identity);
        if expected_mac == mac {
            let ciphertext = axolotl_impl.decode_ciphertext(message);
            axolotl_impl.decrypt_message(&message_key, ciphertext.borrow())
        }
        else {
            None
        }
    }

    fn try_decrypt<'a>(
        &mut self,
        axolotl_impl : &T,
        message_number : usize,
        message : &'a T::Message, 
        mac : T::Mac,
        sender_identity : &T::PublicKey, 
        receiver_identity : &T::PublicKey,
    ) -> Option<T::PlainText> where &'a T::Message : AxolotlMessageRef<T> {
        self.try_get_message_key_index(axolotl_impl, message_number)
            .and_then(|message_key_index| {
                let plaintext = self.try_decrypt_with_message_key_index(
                    axolotl_impl,
                    message, 
                    mac, 
                    sender_identity, 
                    receiver_identity, 
                    message_key_index
                );
                if plaintext.is_some() {
                    self.message_keys.remove(message_key_index);
                }
                plaintext
            })
    }
}
impl <T:Axolotl> AxolotlState<T> {

    pub fn encrypt(&mut self, axolotl_impl : &T, plaintext : &T::PlainText) -> (T::Message, T::Mac) {
        let (new_chain_key,result) = self.encrypt_and_get_next_chain_key(axolotl_impl, plaintext);
        self.chain_key_send = new_chain_key;
        self.message_number_send += 1;
        result
    }

    fn encrypt_and_get_next_chain_key(&self, axolotl_impl : &T, plaintext : &T::PlainText) -> (T::ChainKey, (T::Message, T::Mac)) {
        let (new_chain_key, message_key) = axolotl_impl.derive_next_chain_and_message_key(&self.chain_key_send);
        let ciphertext = axolotl_impl.encrypt_message(&message_key, plaintext);

        let message = axolotl_impl.encode_header_and_ciphertext(
            self.message_number_send,
            self.ratchet_key_send.public.clone(),
            ciphertext
        );
        let mac = axolotl_impl.authenticate_message(&message, &message_key, &self.identity_key_remote, &self.identity_key_local);

        (new_chain_key,(message,mac))
    }

    pub fn decrypt<'a>(&mut self, axolotl_impl : &T, message : &'a T::Message, mac : T::Mac
    ) -> Option<T::PlainText> where &'a T::Message : AxolotlMessageRef<T> {
        let (message_number, message_ratchet_key) = axolotl_impl.decode_header(message);
        let receive_chain_position =  self.receive_chains.iter().position(
            | &ReceiveChain{ref ratchet_key, ..} | axolotl_impl.ratchet_keys_are_equal(ratchet_key, message_ratchet_key.borrow())
        );

        match receive_chain_position {
            Some(pos) => {
                let receive_chain = &mut self.receive_chains[pos];
                receive_chain.try_decrypt(axolotl_impl, message_number, message, mac, &self.identity_key_local, &self.identity_key_remote)
            }
            None => {
                self.try_decrypt_with_new_chain(axolotl_impl, message_number, message_ratchet_key.borrow(), message, mac)
            }
                
        }
    }

    fn try_decrypt_with_new_chain<'a>(
        &mut self, 
        axolotl_impl : &T, 
        message_number : usize,
        message_ratchet_key : &T::PublicKey,
        message : &'a T::Message, 
        mac : T::Mac
    ) -> Option<T::PlainText>  where &'a T::Message : AxolotlMessageRef<T>{
        let ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&self.ratchet_key_send.key, message_ratchet_key);
        let (receiver_root_key, receiver_chain_key) = axolotl_impl.derive_next_root_key_and_chain_key(self.root_key.clone(), &ratchet_key_derive_shared_secret);
        let new_ratchet_key_send = axolotl_impl.generate_ratchet_key_pair();
        let new_ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&new_ratchet_key_send.key, message_ratchet_key);
        let (root_key, chain_key_send) = axolotl_impl.derive_next_root_key_and_chain_key(receiver_root_key, &new_ratchet_key_derive_shared_secret);
        let truncate_to = axolotl_impl.skipped_chain_limit();

        let mut new_receive_chain = ReceiveChain {
            chain_key : receiver_chain_key,
            chain_key_index : 0,
            ratchet_key : message_ratchet_key.clone(),
            message_keys : Vec::new(),
        };

        let plaintext = new_receive_chain.try_decrypt(axolotl_impl, message_number, message, mac, &self.identity_key_local, &self.identity_key_remote);
        if plaintext.is_some() {
            self.receive_chains.insert(0, new_receive_chain);
            self.receive_chains.truncate(truncate_to);
            self.message_number_send = 0;
            self.root_key = root_key;
            self.chain_key_send = chain_key_send;
            self.ratchet_key_send = new_ratchet_key_send;
        }
        plaintext
    }
}