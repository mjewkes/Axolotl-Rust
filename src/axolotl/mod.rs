
use std::option::{Option};
use std::vec::{Vec};

pub trait Axolotl {
	type IdentityKey : DH;
	type RatchetKey : DH;

	type RootKey : Clone;
	type ChainKey : Clone;
	type MessageKey : Clone;

	type PlainText;
	type CipherText;

	fn kdf_initial(ab0 : &<Self::IdentityKey as DH>::Shared, a0b : &<Self::IdentityKey as DH>::Shared, a0b0 : &<Self::IdentityKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey);

	// This is the DH future secrecy ratchet/
	fn kdf_ratchet(root_key : Self::RootKey, ratchet : <Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey);

	//This is the SCIMP style forward secrecy chain key iteration.
	fn kdf_message(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey);

	fn encode_message(message_key : &Self::MessageKey, plaintext : &Self::PlainText) -> Self::CipherText;
	fn decode_message(message_key : &Self::MessageKey, cyphertext : &Self::CipherText) -> Option<Self::PlainText>;

	fn ratchet_keys_are_equal(key0 : &<Self::RatchetKey as DH>::Public, key1 : &<Self::RatchetKey as DH>::Public) -> bool;
	fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey>;

	fn future_message_limit() -> u32;
	fn chain_message_limit() -> u32;

	fn skipped_chain_limit() -> usize;
}

pub trait DH {
	type Private;
	type Public : Clone;
	type Shared;

	fn public(key : &Self::Private) -> Self::Public;
	fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared;
}

pub struct DHKeyPair<T> where T:DH {
	pub key : T::Private,
	pub public : T::Public,
}

pub struct DHExchangedPair<T> where T:DH {
	pub mine : T::Private,
	pub theirs : T::Public,
}

pub struct AxolotlState<T> where T:Axolotl {
	pub root_key : T::RootKey,
	pub identity_key_local  : <T::IdentityKey as DH>::Public,
	pub identity_key_remote : <T::IdentityKey as DH>::Public,
	pub message_number_send : u32,
	pub message_number_recv : u32,
	pub message_number_prev : u32,

	pub chain_key_send : T::ChainKey,
	pub ratchet_key_send : DHKeyPair<T::RatchetKey>,

	pub receive_chains : Vec<ReceiveChain<T>>,

	pub ratchet_flag : bool,

}

pub struct ReceiveChain<T> where T:Axolotl {
	pub chain_key : T::ChainKey,
	pub chain_key_index : u32,
	pub ratchet_key : <T::RatchetKey as DH>::Public,
	pub message_keys : Vec<(u32,T::MessageKey)>,
}

pub struct AxolotlHeader<T> where T:Axolotl {
	pub message_number : u32,
	pub message_number_prev : u32,
	pub ratchet_key : <T::RatchetKey as DH>::Public,
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
			let (next_chain_key, message_key) = T::kdf_message(&self.chain_key);
			self.chain_key = next_chain_key;
			self.message_keys.push((i,message_key));
		}
		self.chain_key_index = index;

		return Some(self.message_keys.pop().unwrap().1);
	}
}
impl <T:Axolotl> AxolotlState<T> {

	pub fn encrypt(&mut self, plaintext : &T::PlainText) -> (AxolotlHeader<T>, T::CipherText) {
		let (new_chain_key, message_key) = T::kdf_message(&self.chain_key_send);
		let ciphertext = T::encode_message(&message_key, plaintext);

		let header = AxolotlHeader {
			message_number : self.message_number_send,
			message_number_prev : self.message_number_prev,
			ratchet_key : self.ratchet_key_send.public.clone(),
		};
		self.chain_key_send = new_chain_key;
		self.message_number_send += 1;

		(header, ciphertext)
	}
	pub fn decrypt(&mut self, header : &AxolotlHeader<T>, ciphertext : &T::CipherText) -> Option<T::PlainText> {
		let receive_chain = self.get_or_create_receive_chain(&header.ratchet_key);
		let message_key_or_none = receive_chain.get_or_create_message_key(header.message_number);

		if let None = message_key_or_none  {
			return None;
		}
		let message_key = message_key_or_none.unwrap();

		T::decode_message(&message_key, ciphertext)
	}

	fn get_or_create_receive_chain(&mut self, ratchet_key_theirs : &<T::RatchetKey as DH>::Public) -> &mut ReceiveChain<T> {
		//TODO: comment on why this is, for loop early return breaks borrowing
		let receive_chain_position =  self.receive_chains.iter().position(
			| &ReceiveChain{ref ratchet_key, ..} | T::ratchet_keys_are_equal(ratchet_key, &ratchet_key_theirs)
			);

		match receive_chain_position {
			Some(pos) => {
				&mut self.receive_chains[pos]
			}
			None => {
				let ratchet_key_shared = <T::RatchetKey as DH>::shared(&self.ratchet_key_send.key, &ratchet_key_theirs);
				let (receiver_root_key, receiver_chain_key) = T::kdf_ratchet(self.root_key.clone(), ratchet_key_shared);
				let new_ratchet_key_send = T::generate_ratchet_key_pair();
				let new_ratchet_key_shared = <T::RatchetKey as DH>::shared(&new_ratchet_key_send.key, &ratchet_key_theirs);
				let (root_key, chain_key_send) = T::kdf_ratchet(receiver_root_key, new_ratchet_key_shared);
		
				let new_receive_chain = ReceiveChain {
					chain_key : receiver_chain_key,
					chain_key_index : 0,
					ratchet_key : ratchet_key_theirs.clone(),
					message_keys : Vec::new(),
				};
		
				let truncate_to = T::skipped_chain_limit();

				self.receive_chains.insert(0, new_receive_chain);
				self.receive_chains.truncate(truncate_to);
				self.message_number_prev = self.message_number_send;
				self.message_number_send = 0;
				self.root_key = root_key;
				self.chain_key_send = chain_key_send;
				self.ratchet_key_send = new_ratchet_key_send;
				&mut self.receive_chains[0]
			}
		}
	}
}