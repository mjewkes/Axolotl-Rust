use super::axolotl::{Axolotl};
use super::dh::{DH,DHExchangedPair,DHKeyPair,DHPublic};
use super::message::{AxolotlMessage};

pub struct AxolotlState<T> where T:Axolotl {
    root_key : T::RootKey,
    identity_key_local  : DHPublic<T::IdentityKey>,
    identity_key_remote : DHPublic<T::IdentityKey>,
    message_number_send : u32,

    chain_key_send : T::ChainKey,
    ratchet_key_send : DHKeyPair<T::RatchetKey>,

    receive_chains : Vec<ReceiveChain<T>>,
}

struct ReceiveChain<T> where T:Axolotl {
    chain_key : T::ChainKey,
    chain_key_index : u32,
    ratchet_key : DHPublic<T::RatchetKey>,
    message_keys : Vec<(u32,T::MessageKey)>,
}



pub fn init_as_alice<T>(
    identity_keys : &DHExchangedPair<T::IdentityKey>, 
    handshake_keys : &DHExchangedPair<T::IdentityKey>, 
    initial_ratchet_key : &DHPublic<T::RatchetKey>) 
-> AxolotlState<T> 
    where T:Axolotl {
        let ratchet_keypair = T::generate_ratchet_key_pair();
        init_as_alice_with_explicit_ratchet_keypair(identity_keys,handshake_keys,ratchet_keypair,initial_ratchet_key)

}
pub fn init_as_alice_with_explicit_ratchet_keypair<T>(
    identity_keys : &DHExchangedPair<T::IdentityKey>, 
    handshake_keys : &DHExchangedPair<T::IdentityKey>, 
    my_ratchet_keypair : DHKeyPair<T::RatchetKey>, 
    initial_ratchet_key : &DHPublic<T::RatchetKey>) 
-> AxolotlState<T> 
    where T:Axolotl {
        let ab0 = <T::IdentityKey as DH>::shared(&identity_keys.mine, &handshake_keys.theirs);
        let a0b = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &identity_keys.theirs);
        let a0b0 = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &handshake_keys.theirs);
        let (pre_root_key, chain_key_recv) = T::derive_initial_root_key_and_chain_key(&ab0, &a0b, &a0b0);
        let ratchet_key = my_ratchet_keypair;
        let ratchet_key_shared = <T::RatchetKey as DH>::shared(&ratchet_key.key, initial_ratchet_key);
        let (root_key,chain_key_send) = T::derive_next_root_key_and_chain_key(pre_root_key, &ratchet_key_shared);
        let initial_receive_chain = ReceiveChain {
            chain_key : chain_key_recv,
            chain_key_index : 0,
            ratchet_key : initial_ratchet_key.clone(),
            message_keys : Vec::new(),
        };
        AxolotlState {
            root_key : root_key,
            identity_key_local : <T::IdentityKey as DH>::public(&identity_keys.mine),
            identity_key_remote : identity_keys.theirs.clone(),
            message_number_send : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : ratchet_key,
            receive_chains : vec![initial_receive_chain],
        }
}

pub fn init_as_bob<T>(
    identity_keys : &DHExchangedPair<T::IdentityKey>, 
    handshake_keys : &DHExchangedPair<T::IdentityKey>, 
    initial_ratchet_key : DHKeyPair<T::RatchetKey>) 
-> AxolotlState<T> 
    where T:Axolotl {
        let ab0 = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &identity_keys.theirs);
        let a0b = <T::IdentityKey as DH>::shared(&identity_keys.mine, &handshake_keys.theirs);
        let a0b0 = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &handshake_keys.theirs);
        let (root_key, chain_key_send) = T::derive_initial_root_key_and_chain_key(&ab0, &a0b, &a0b0);
        AxolotlState {
            root_key : root_key,
            identity_key_local : <T::IdentityKey as DH>::public(&identity_keys.mine),
            identity_key_remote : identity_keys.theirs.clone(),
            message_number_send : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : initial_ratchet_key,
            receive_chains : Vec::new(),
        }
}



impl <T:Axolotl> ReceiveChain<T> {
    fn try_get_message_key_index(&mut self, index : u32) -> Option<usize> {
        if index > T::chain_message_limit() {
            return None;
        }
        //TODO: make sure this doesn't overflow
        if index > self.chain_key_index + T::future_message_limit() {
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
            let (next_chain_key, message_key) = T::derive_next_chain_and_message_key(&self.chain_key);
            self.message_keys.push((i,message_key));
            self.chain_key = next_chain_key;
            self.chain_key_index += 1;
        }

        return Some(self.message_keys.len()-1);
    }

    fn try_decrypt_with_message_key_index(
        &self,
        message : &AxolotlMessage<T>, 
        mac : T::Mac,
        sender_identity : &DHPublic<T::IdentityKey>, 
        receiver_identity : &DHPublic<T::IdentityKey>,
        message_key_index : usize,
    ) -> Option<T::PlainText> {
        let (_,ref message_key) = self.message_keys[message_key_index];
        let expected_mac = T::authenticate_message(message, message_key, sender_identity, receiver_identity);
        if expected_mac == mac {
            T::decrypt_message(&message_key, &message.ciphertext)
        }
        else {
            None
        }
    }

    fn try_decrypt(
        &mut self,
        message : &AxolotlMessage<T>, 
        mac : T::Mac,
        sender_identity : &DHPublic<T::IdentityKey>, 
        receiver_identity : &DHPublic<T::IdentityKey>,
    ) -> Option<T::PlainText> {
        self.try_get_message_key_index(message.message_number)
            .and_then(|message_key_index| {
                let plaintext = self.try_decrypt_with_message_key_index(
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

    pub fn encrypt(&mut self, plaintext : &T::PlainText) -> (AxolotlMessage<T>, T::Mac) {
        let (new_chain_key,result) = self.encrypt_and_get_next_chain_key(plaintext);
        self.chain_key_send = new_chain_key;
        self.message_number_send += 1;
        result
    }

    fn encrypt_and_get_next_chain_key(&self, plaintext : &T::PlainText) -> (T::ChainKey, (AxolotlMessage<T>, T::Mac)) {
        let (new_chain_key, message_key) = T::derive_next_chain_and_message_key(&self.chain_key_send);
        let ciphertext = T::encrypt_message(&message_key, plaintext);

        let message = AxolotlMessage {
            message_number : self.message_number_send,
            ratchet_key : self.ratchet_key_send.public.clone(),
            ciphertext : ciphertext
        };
        let mac = T::authenticate_message(&message, &message_key, &self.identity_key_remote, &self.identity_key_local);

        (new_chain_key,(message,mac))
    }

    pub fn decrypt(&mut self, message : &AxolotlMessage<T>, mac : T::Mac) -> Option<T::PlainText> {
        let receive_chain_position =  self.receive_chains.iter().position(
            | &ReceiveChain{ref ratchet_key, ..} | T::ratchet_keys_are_equal(ratchet_key, &message.ratchet_key)
        );

        match receive_chain_position {
            Some(pos) => {
                let receive_chain = &mut self.receive_chains[pos];
                receive_chain.try_decrypt(message, mac, &self.identity_key_local, &self.identity_key_remote)
            }
            None => {
                self.try_decrypt_with_new_chain(message, mac)
            }
                
        }
    }

    fn try_decrypt_with_new_chain(&mut self, message : &AxolotlMessage<T>, mac : T::Mac) -> Option<T::PlainText> {
        let ratchet_key_shared = <T::RatchetKey as DH>::shared(&self.ratchet_key_send.key, &message.ratchet_key);
        let (receiver_root_key, receiver_chain_key) = T::derive_next_root_key_and_chain_key(self.root_key.clone(), &ratchet_key_shared);
        let new_ratchet_key_send = T::generate_ratchet_key_pair();
        let new_ratchet_key_shared = <T::RatchetKey as DH>::shared(&new_ratchet_key_send.key, &message.ratchet_key);
        let (root_key, chain_key_send) = T::derive_next_root_key_and_chain_key(receiver_root_key, &new_ratchet_key_shared);
        let truncate_to = T::skipped_chain_limit();

        let mut new_receive_chain = ReceiveChain {
            chain_key : receiver_chain_key,
            chain_key_index : 0,
            ratchet_key : message.ratchet_key.clone(),
            message_keys : Vec::new(),
        };

        let plaintext = new_receive_chain.try_decrypt(message, mac, &self.identity_key_local, &self.identity_key_remote);
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