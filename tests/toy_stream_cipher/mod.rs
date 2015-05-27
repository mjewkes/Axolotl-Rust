extern crate rand;
extern crate raxolotl;

use self::rand::*;
use self::raxolotl::axolotl::*;

static mut rng_seed : u64 = 0;
fn get_rng(seed : u64) -> ChaChaRng {
    let mut rng = ChaChaRng::new_unseeded();
    rng.set_counter(0, seed);
    rng
}
fn next_rng() -> ChaChaRng {
    let seed;
    unsafe {
        seed = rng_seed;
        rng_seed += 1;
    }
    get_rng(seed)
}

pub struct Substitution;

pub struct Message {
    pub message_number : usize,
    pub ratchet_key : u64,
    pub ciphertext : Vec<u8>,
}

impl<'a> AxolotlMessageRef<Substitution> for &'a Message {
    type RatchetKey = &'a u64;
    type CipherText = &'a Vec<u8>;
}

impl Axolotl for Substitution {
    type PrivateKey = u64;
    type PublicKey = u64;
    type SharedSecret = u64;

    type RootKey = u64;
    type ChainKey = u64;
    type MessageKey = u64;

    type PlainText = Vec<u8>;
    type CipherText = Vec<u8>;
    type Message = Message;

    type Mac = ();

    fn derive_initial_root_key_and_chain_key(&self, a : &u64, b : &u64, c : &u64) -> (u64,u64) {
        let seed = *a ^ b.wrapping_mul(31) ^ c.wrapping_mul(31*31);
        let mut rng = get_rng(seed);
        let root_key = rng.next_u64();
        let chain_key = rng.next_u64();
        (root_key,chain_key)
    }

    // This is the DH future secrecy ratchet/
    fn derive_next_root_key_and_chain_key(&self, a : u64, b : &u64) -> (u64,u64) {
        let seed = a ^ *b;
        let mut rng = get_rng(seed);
        let root_key = rng.next_u64();
        let chain_key = rng.next_u64();
        (root_key,chain_key)
    }


    //This is the SCIMP style forward secrecy chain key iteration.
    fn derive_next_chain_and_message_key(&self, a : &u64) -> (u64,u64) {
        let mut rng = get_rng(*a);
        let chain_key = rng.next_u64();
        let message_key = rng.next_u64();
        (chain_key,message_key)
    }


    fn encrypt_message(
        &self,
        key : &u64,
        plaintext : &Vec<u8>) 
    -> Vec<u8> {
        let mut rng = get_rng(*key);
        plaintext
            .iter()
            .map(|b|{rng.gen::<u8>() ^ b})
            .collect()
    }


    fn decrypt_message(
        &self,
        key : &u64,
        ciphertext : &Vec<u8>) 
    -> Option<Vec<u8>> {
        let mut rng = get_rng(*key);
        let plaintext = ciphertext
            .iter()
            .map(|b| {rng.gen::<u8>() ^ b})
            .collect();
        Some(plaintext)
    }

    fn encode_header_and_ciphertext(
        &self, 
        message_number : usize, 
        ratchet_key : Self::PublicKey, 
        ciphertext : Self::CipherText
    ) -> Self::Message {
        Message {
            message_number : message_number,
            ratchet_key : ratchet_key,
            ciphertext : ciphertext,
        }
    }

    fn decode_header<'a>(&self, message : &'a Self::Message
    ) -> (usize, &'a Self::PublicKey) {
        (message.message_number, &message.ratchet_key)
    }

    fn decode_ciphertext<'a>(&self, message : &'a Self::Message
    ) -> &'a Self::CipherText {
        &message.ciphertext
    }

    fn authenticate_message(&self, _ : &Self::Message, _ : &u64, _ : &u64, _ : &u64) {
    }

    fn ratchet_keys_are_equal(&self, a : &u64, b : &u64) -> bool {
        *a == *b
    }

    fn generate_ratchet_key_pair(&self) -> KeyPair<Self> {
        let key = next_rng().next_u64();
        let public = self.derive_public_key(&key);
        KeyPair{ key : key, public : public }
    }

    fn derive_public_key(&self,key : &u64) -> u64 {
        key.wrapping_mul(31)
    }

    fn derive_shared_secret(&self,a : &u64, b : &u64) -> u64 {
        a.wrapping_mul(*b)
    }


    fn future_message_limit(&self) -> usize {
        1024
    }

    fn chain_message_limit(&self) -> usize {
        1024
    }


    fn skipped_chain_limit(&self) -> usize {
        8
    }
}

pub fn init_alice_and_bob(axolotl_impl : &Substitution) -> (AxolotlState<Substitution>, AxolotlState<Substitution>) {
    let alice_identity = axolotl_impl.generate_ratchet_key_pair();
    let alice_handshake = axolotl_impl.generate_ratchet_key_pair();
    let bob_identity = axolotl_impl.generate_ratchet_key_pair();
    let bob_handshake = axolotl_impl.generate_ratchet_key_pair();
    let initial_ratchet = axolotl_impl.generate_ratchet_key_pair();

    let alice = init_as_alice::<Substitution>(
        axolotl_impl, 
        &alice_identity.key, 
        &bob_identity.public,
        &alice_handshake.key,
        &bob_handshake.public, 
        &initial_ratchet.public
    );
    let bob = init_as_bob::<Substitution>(
        axolotl_impl,
        &bob_identity.key,
        &alice_identity.public,
        &bob_handshake.key,
        &alice_handshake.public,
        initial_ratchet
    );

    (alice,bob)
}

pub fn check_send(axolotl_impl : &Substitution, sender : &mut AxolotlState<Substitution>, receiver : &mut AxolotlState<Substitution>, message : String) -> Message {
    let m = message.into_bytes();
    let encrypted = sender.encrypt(axolotl_impl, &m);
    let decrypted = receiver.decrypt(axolotl_impl, &encrypted.0, encrypted.1).unwrap();
    assert!(m[..] == decrypted[..]);
    assert!(m[..] != encrypted.0.ciphertext[..]);
    encrypted.0
}