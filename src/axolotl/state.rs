use std::collections::HashMap;
use std::result::Result;
use super::axolotl::{Axolotl, Header, KeyPair, SendError, ReceiveError};

pub struct AxolotlState<T> where T:Axolotl {
    root_key : T::RootKey,
    session_identity : T::SessionIdentity,
    message_number_send : usize,
    message_number_prev : usize,

    chain_key_send : T::ChainKey,
    ratchet_key_send : KeyPair<T>,

    skipped_receive_chains : Vec<ReceiveChain<T>>,
    current_receive_chain : Option<(ReceiveChain<T>, T::ChainKey)>,
}

struct ReceiveChain<T> where T:Axolotl {
    next_chain_key_index : usize,
    ratchet_key : T::PublicKey,
    message_keys : HashMap<usize,T::MessageKey>,
}
impl<T:Axolotl> Clone for ReceiveChain<T> {
    fn clone(&self) -> Self {
        ReceiveChain{
            next_chain_key_index : self.next_chain_key_index,
            ratchet_key : self.ratchet_key.clone(),
            message_keys : self.message_keys.clone(),
        }
    }
}



pub fn init_as_alice<T>(
    axolotl_impl : &T,
    session_identity : T::SessionIdentity,
    initial_secret : T::InitialSharedSecret,
    bob_ratchet_key_send : T::PublicKey
    ) -> AxolotlState<T> 
    where T:Axolotl {
        let ratchet_keypair = axolotl_impl.generate_ratchet_key_pair();
        init_as_alice_with_explicit_ratchet_keypair(
            axolotl_impl,
            session_identity,
            initial_secret,
            ratchet_keypair,
            bob_ratchet_key_send
        ) 

}
pub fn init_as_alice_with_explicit_ratchet_keypair<T>(
    axolotl_impl : &T,
    session_identity : T::SessionIdentity,
    initial_secret : T::InitialSharedSecret,
    my_ratchet_keypair : KeyPair<T>,
    bob_ratchet_key_send : T::PublicKey) 
-> AxolotlState<T> 
    where T:Axolotl {
        let (pre_root_key, chain_key_recv) = axolotl_impl.derive_initial_root_key_and_chain_key(initial_secret);
        let ratchet_key = my_ratchet_keypair;
        let ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&ratchet_key.key, &bob_ratchet_key_send);
        let (root_key,chain_key_send) = axolotl_impl.derive_next_root_key_and_chain_key(pre_root_key, &ratchet_key_derive_shared_secret);
        let initial_receive_chain = ReceiveChain {
            next_chain_key_index : 0,
            ratchet_key : bob_ratchet_key_send,
            message_keys : HashMap::new(),
        };
        AxolotlState {
            root_key : root_key,
            session_identity : session_identity,
            message_number_send : 0,
            message_number_prev : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : ratchet_key,
            skipped_receive_chains : Vec::new(),
            current_receive_chain : Some((initial_receive_chain, chain_key_recv)),
        }
}

pub fn init_as_bob<T>(
    axolotl_impl : &T,
    session_identity : T::SessionIdentity,
    initial_secret : T::InitialSharedSecret,
    bob_ratchet_key_send : KeyPair<T>) 
-> AxolotlState<T> 
    where T:Axolotl {
        let (root_key, chain_key_send) = axolotl_impl.derive_initial_root_key_and_chain_key(initial_secret);
        AxolotlState {
            root_key : root_key,
            session_identity : session_identity,
            message_number_send : 0,
            message_number_prev : 0,
            chain_key_send : chain_key_send,
            ratchet_key_send : bob_ratchet_key_send,
            skipped_receive_chains : Vec::new(),
            current_receive_chain : None,
        }
}



impl <T:Axolotl> ReceiveChain<T> {
    fn find_message_key(&self, message_number : usize) -> Result<&T::MessageKey,ReceiveError<T>> {
        if message_number >= self.next_chain_key_index {
            return Err(ReceiveError::MessageNumberAheadOfChainLength(message_number));
        }

        match self.message_keys.get(&message_number) {
            Some(key) => Ok(key),
            None => Err(ReceiveError::MessageNumberAlreadyUsed(message_number)),
        }
    }

    fn create_message_keys(&mut self, axolotl_impl : &T, chain_key : &mut T::ChainKey, index : usize) -> Result<(), ReceiveError<T>> {
        if index > axolotl_impl.chain_message_limit() {
            return Err(ReceiveError::MessageNumberTooLarge(index));
        }

        if index > self.next_chain_key_index + axolotl_impl.future_message_limit() {
            return Err(ReceiveError::MessageNumberTooFarAhead(index));
        }

        
        for i in self.next_chain_key_index..(index+1) {
            let (next_chain_key, message_key) = axolotl_impl.derive_next_chain_and_message_key(chain_key);
            self.message_keys.insert(i,message_key);
            *chain_key = next_chain_key;
            self.next_chain_key_index += 1;
        }
        Ok(())
    }

    fn try_authenticate(
        axolotl_impl : &T,
        expected_mac : &T::Mac,
        message : &T::Message,
        message_key : &T::MessageKey,
        session_identity : &T::SessionIdentity
    ) -> Result<(),ReceiveError<T>> {
        let received_mac = axolotl_impl.authenticate_message(message, message_key, session_identity);
        if expected_mac != &received_mac {
            Err(ReceiveError::InvalidMac)
        }else {
            Ok(())
        }
    }

    fn decode_and_decrypt_message(
        axolotl_impl : &T,
        message : T::Message,
        message_key : &T::MessageKey
    ) -> Result<T::PlainText,ReceiveError<T>>{
        let ciphertext = try!(axolotl_impl.decode_ciphertext(message).map_err(|e|{ReceiveError::DecodeError(e)}));
        axolotl_impl.decrypt_message(&message_key, ciphertext).map_err(|e|{ReceiveError::DecryptError(e)})
    }

    fn try_decrypt(
        &mut self,
        axolotl_impl : &T,
        message_number : usize,
        message : T::Message, 
        mac : &T::Mac,
        session_identity : &T::SessionIdentity,
    ) -> Result<T::PlainText,ReceiveError<T>> {
        let plaintext = {
            let ref message_key = try!(self.find_message_key(message_number));
            try!(ReceiveChain::try_authenticate(axolotl_impl, mac,&message,message_key,session_identity));
            ReceiveChain::decode_and_decrypt_message(axolotl_impl,message,message_key)
        };
        
        if plaintext.is_ok() {
            self.message_keys.remove(&message_number);
        }
        plaintext
    }

    fn try_create_keys_and_decrypt(
        &mut self,
        axolotl_impl : &T,
        chain_key : &mut T::ChainKey,
        message_number : usize,
        message : T::Message, 
        mac : &T::Mac,
        session_identity : &T::SessionIdentity
    ) -> Result<T::PlainText,ReceiveError<T>> {
        try!(self.create_message_keys(axolotl_impl, chain_key, message_number));
        self.try_decrypt(axolotl_impl, message_number, message, mac, session_identity)
    }
}
impl <T:Axolotl> AxolotlState<T> {

    pub fn encrypt(&mut self, axolotl_impl : &T, plaintext : T::PlainText) -> Result<(T::Message, T::Mac), SendError<T>> {
        let (new_chain_key,result) = try!(self.encrypt_and_get_next_chain_key(axolotl_impl, plaintext));
        self.chain_key_send = new_chain_key;
        self.message_number_send += 1;
        Ok(result)
    }

    fn encrypt_and_get_next_chain_key(&self, axolotl_impl : &T, plaintext : T::PlainText) -> Result<(T::ChainKey, (T::Message, T::Mac)), SendError<T>> {
        let (new_chain_key, message_key) = axolotl_impl.derive_next_chain_and_message_key(&self.chain_key_send);
        let ciphertext = try!(axolotl_impl.encrypt_message(&message_key, plaintext).map_err(|e|SendError::EncryptError(e)));

        let message = try!(axolotl_impl.encode_header_and_ciphertext(
            Header{
                message_number : self.message_number_send,
                message_number_prev : self.message_number_prev,
                ratchet_key : self.ratchet_key_send.public.clone(),
            },
            ciphertext
        ).map_err(|e|SendError::EncodeError(e)));

        let mac = axolotl_impl.authenticate_message(&message, &message_key, &self.session_identity);

        Ok((new_chain_key,(message,mac)))
    }

    pub fn decrypt(&mut self, axolotl_impl : &T, message : T::Message, ref mac : T::Mac
    ) -> Result<T::PlainText,ReceiveError<T>> {
        let Header{ message_number, message_number_prev, ratchet_key : ref message_ratchet_key } = try!(
            axolotl_impl
                .decode_header(&message)
                .map_err(|e|{ReceiveError::DecodeError(e)})
        );

        if let Some(skipped_chain_index) = self.find_skipped_chain_index(axolotl_impl, message_ratchet_key) {
            return self.try_decrypt_with_skipped_chain(axolotl_impl, message, mac, message_number, skipped_chain_index);
        }

        if self.current_chain_matches_ratchet_key(axolotl_impl, message_ratchet_key) {
            return self.try_decrypt_with_current_chain(axolotl_impl, message, mac, message_number, message_ratchet_key);
        }

        self.try_decrypt_with_new_chain(axolotl_impl, message_number, message_number_prev, message_ratchet_key, message, mac)
    }

    fn find_skipped_chain_index(
        &self, 
        axolotl_impl : &T,
        message_ratchet_key : &T::PublicKey
    ) -> Option<usize> {
        self.skipped_receive_chains.iter().position(
            | &ReceiveChain{ref ratchet_key, ..} | axolotl_impl.ratchet_keys_are_equal(ratchet_key, message_ratchet_key)
        )
    }

    fn try_decrypt_with_skipped_chain(
        &mut self, 
        axolotl_impl : &T, 
        message : T::Message, 
        mac : &T::Mac, 
        message_number : usize, 
        skipped_chain_index : usize,
    ) ->  Result<T::PlainText,ReceiveError<T>> {
        let receive_chain = &mut self.skipped_receive_chains[skipped_chain_index];
        receive_chain.try_decrypt(axolotl_impl, message_number, message, mac, &self.session_identity)
    }

    fn current_chain_matches_ratchet_key(
        &self, 
        axolotl_impl : &T,
        message_ratchet_key : &T::PublicKey
    ) -> bool {
        match self.current_receive_chain {
            None => false,
            Some((ref current_chain, _)) => axolotl_impl.ratchet_keys_are_equal(&current_chain.ratchet_key, message_ratchet_key),
        }
    }

    fn try_decrypt_with_current_chain(
        &mut self, 
        axolotl_impl : &T, 
        message : T::Message, 
        mac : &T::Mac, 
        message_number : usize, 
        message_ratchet_key : &T::PublicKey
    ) ->  Result<T::PlainText,ReceiveError<T>> {
        let &mut (ref mut current_chain, ref mut chain_key) = self.current_receive_chain.as_mut().unwrap();
        assert!(axolotl_impl.ratchet_keys_are_equal(&current_chain.ratchet_key, message_ratchet_key));
        current_chain.try_create_keys_and_decrypt(axolotl_impl, chain_key, message_number, message, mac, &self.session_identity)
    }

    fn try_decrypt_with_new_chain(
        &mut self, 
        axolotl_impl : &T, 
        message_number : usize,
        message_number_prev : usize,
        message_ratchet_key : &T::PublicKey,
        message : T::Message, 
        mac : &T::Mac
    ) -> Result<T::PlainText,ReceiveError<T>> {
        let ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&self.ratchet_key_send.key, message_ratchet_key);
        let (receiver_root_key, mut receiver_chain_key) = axolotl_impl.derive_next_root_key_and_chain_key(self.root_key.clone(), &ratchet_key_derive_shared_secret);

        let mut new_receive_chain = ReceiveChain {
            next_chain_key_index : 0,
            ratchet_key : message_ratchet_key.clone(),
            message_keys : HashMap::new(),
        };
        try!(new_receive_chain.create_message_keys(axolotl_impl, &mut receiver_chain_key, message_number));
        let plaintext = try!(new_receive_chain.try_decrypt(axolotl_impl, message_number, message, mac, &self.session_identity));

        try!(self.advance_ratchet(axolotl_impl, message_ratchet_key, receiver_root_key, new_receive_chain, receiver_chain_key, message_number_prev));

        Ok(plaintext)
    }

    fn advance_ratchet(
        &mut self, 
        axolotl_impl : &T, 
        message_ratchet_key : &T::PublicKey, 
        receiver_root_key : T::RootKey,
        new_receive_chain : ReceiveChain<T>,
        receiver_chain_key : T::ChainKey,
        message_number_prev : usize
    ) -> Result<(), ReceiveError<T>> {
        let new_ratchet_key_send = axolotl_impl.generate_ratchet_key_pair();
        let new_ratchet_key_derive_shared_secret = axolotl_impl.derive_shared_secret(&new_ratchet_key_send.key, message_ratchet_key);
        let (root_key, chain_key_send) = axolotl_impl.derive_next_root_key_and_chain_key(receiver_root_key, &new_ratchet_key_derive_shared_secret);
        let truncate_to = axolotl_impl.skipped_chain_limit();
        if let Some((ref mut current_chain, ref mut current_chain_key)) = self.current_receive_chain {
            try!(current_chain.create_message_keys(axolotl_impl, current_chain_key, message_number_prev));
            self.skipped_receive_chains.insert(0, current_chain.clone());
            self.skipped_receive_chains.truncate(truncate_to);
        }
        self.current_receive_chain = Some((new_receive_chain, receiver_chain_key));
        self.message_number_prev = self.message_number_send;
        self.message_number_send = 0;
        self.root_key = root_key;
        self.chain_key_send = chain_key_send;
        self.ratchet_key_send = new_ratchet_key_send;
        Ok(())
    }
}