use axolotl::{Axolotl, AxolotlMessage, DH, DHExchangedPair, DHKeyPair, DHPublic};
use std::borrow::ToOwned;

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

impl <T:Axolotl> Clone for AxolotlState<T> {
    fn clone(&self) -> Self {
        AxolotlState {
            root_key : self.root_key.clone(),
            identity_key_local : self.identity_key_local.clone(),
            identity_key_remote : self.identity_key_remote.clone(),
            message_number_send : self.message_number_send,
            chain_key_send : self.chain_key_send.clone(),
            ratchet_key_send : Clone::clone(&self.ratchet_key_send),
            receive_chains : self.receive_chains.clone(),
        }
    }
}

impl <T:Axolotl> Clone for ReceiveChain<T> {
    fn clone(&self) -> Self {
        ReceiveChain {
            chain_key : self.chain_key.clone(),
            chain_key_index : self.chain_key_index,
            ratchet_key : self.ratchet_key.clone(),
            message_keys : self.message_keys.clone(),
        }
    }
}


pub fn init_as_alice<T>(
    identity_keys : &DHExchangedPair<T::IdentityKey>,
    handshake_keys : &DHExchangedPair<T::IdentityKey>,
    initial_ratchet_key : &DHPublic<T::RatchetKey>)
-> AxolotlState<T>
    where T:Axolotl {
        let ab0 = <T::IdentityKey as DH>::shared(&identity_keys.mine, &handshake_keys.theirs);
        let a0b = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &identity_keys.theirs);
        let a0b0 = <T::IdentityKey as DH>::shared(&handshake_keys.mine, &handshake_keys.theirs);
        let (pre_root_key, chain_key_recv) = T::derive_initial_root_key_and_chain_key(&ab0, &a0b, &a0b0);
        let ratchet_key = T::generate_ratchet_key_pair();
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
    fn get_or_create_message_key(&mut self, index : u32) -> Option<T::MessageKey> {
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
                    let (_, message_key) = self.message_keys.remove(i);
                    return Some(message_key);
                }
            }
            return None;
        }

        for i in self.chain_key_index..(index+1) {
            let (next_chain_key, message_key) = T::derive_next_chain_and_message_key(&self.chain_key);
            self.chain_key = next_chain_key;
            self.message_keys.push((i,message_key));
        }
        self.chain_key_index = index+1;

        return Some(self.message_keys.pop().unwrap().1);
    }
}

impl <T: Axolotl> AxolotlState<T> {
    pub fn encrypt<P: AsRef<T::PlainText>>(&mut self, plaintext : P) -> (AxolotlMessage<T>, T::Mac) {
        let (new_chain_key, message_key) = T::derive_next_chain_and_message_key(&self.chain_key_send);
        let ciphertext = T::encrypt_message(&message_key, plaintext);

        let message = AxolotlMessage {
            message_number : self.message_number_send,
            ratchet_key : self.ratchet_key_send.public.clone(),
            ciphertext : ciphertext
        };
        let mac = T::authenticate_message(&message, &message_key, &self.identity_key_remote, &self.identity_key_local);
        self.chain_key_send = new_chain_key;
        self.message_number_send += 1;

        (message, mac)
    }

    pub fn decrypt(&mut self, message : &AxolotlMessage<T>, mac : T::Mac)
                    -> Option<<T::PlainText as ToOwned>::Owned> {
        let mut self_clone = Clone::clone(self);
        let result = self_clone.try_decrypt(message, mac);

        if let Some(_) = result {
            *self = self_clone
        }

        result
    }

    fn try_decrypt(&mut self, message : &AxolotlMessage<T>, mac : T::Mac)
                    -> Option<<T::PlainText as ToOwned>::Owned> {
        let message_key = {
            let receive_chain = self.get_or_create_receive_chain(&message.ratchet_key);
            receive_chain.get_or_create_message_key(message.message_number)
        };

        message_key.and_then(|message_key| {
            let expected_mac = T::authenticate_message(message,
                                                       &message_key,
                                                       &self.identity_key_local,
                                                       &self.identity_key_remote);
            if expected_mac == mac {
                T::decrypt_message(&message_key, &message.ciphertext)
            } else {
                None
            }
        })
    }

    fn get_or_create_receive_chain(&mut self, ratchet_key_theirs : &<T::RatchetKey as DH>::Public) -> &mut ReceiveChain<T> {
        //TODO: comment on why this is, for loop early return breaks borrowing
        let receive_chain_position = self.receive_chains
                                         .iter()
                                         .position(|c| c.ratchet_key == *ratchet_key_theirs);

        match receive_chain_position {
            Some(pos) => &mut self.receive_chains[pos],
            None => {
                let ratchet_key_shared = <T::RatchetKey as DH>::shared(&self.ratchet_key_send.key, &ratchet_key_theirs);
                let (receiver_root_key, receiver_chain_key) = T::derive_next_root_key_and_chain_key(self.root_key.clone(), &ratchet_key_shared);
                let new_ratchet_key_send = T::generate_ratchet_key_pair();
                let new_ratchet_key_shared = <T::RatchetKey as DH>::shared(&new_ratchet_key_send.key, &ratchet_key_theirs);
                let (root_key, chain_key_send) = T::derive_next_root_key_and_chain_key(receiver_root_key, &new_ratchet_key_shared);

                let new_receive_chain = ReceiveChain {
                    chain_key : receiver_chain_key,
                    chain_key_index : 0,
                    ratchet_key : ratchet_key_theirs.clone(),
                    message_keys : Vec::new(),
                };

                let truncate_to = T::skipped_chain_limit();

                self.receive_chains.insert(0, new_receive_chain);
                self.receive_chains.truncate(truncate_to);
                self.message_number_send = 0;
                self.root_key = root_key;
                self.chain_key_send = chain_key_send;
                self.ratchet_key_send = new_ratchet_key_send;
                &mut self.receive_chains[0]
            }
        }
    }
}
