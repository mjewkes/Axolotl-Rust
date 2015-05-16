use rand::*;
use raxolotl::axolotl::*;

pub struct DHKey;

impl DH for DHKey {
    type Public = u64;
    type Private = u64;
    type Shared = u64;

    fn public(key : &u64) -> u64 {
        key.wrapping_mul(31)
    }

    fn shared(a : &u64, b : &u64) -> u64 {
        a.wrapping_mul(*b)
    }
}

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

impl Axolotl for Substitution {
    type IdentityKey = DHKey;
    type RatchetKey = DHKey;

    type RootKey = u64;
    type ChainKey = u64;
    type MessageKey = u64;

    type PlainText = Vec<u8>;
    type CipherText = Vec<u8>;

    type Mac = ();

    fn derive_initial_root_key_and_chain_key(a : &u64, b : &u64, c : &u64) -> (u64,u64) {
        let seed = *a ^ b.wrapping_mul(31) ^ c.wrapping_mul(31*31);
        let mut rng = get_rng(seed);
        let root_key = rng.next_u64();
        let chain_key = rng.next_u64();
        (root_key,chain_key)
    }

    // This is the DH future secrecy ratchet/
    fn derive_next_root_key_and_chain_key(a : u64, b : &u64) -> (u64,u64) {
        let seed = a ^ *b;
        let mut rng = get_rng(seed);
        let root_key = rng.next_u64();
        let chain_key = rng.next_u64();
        (root_key,chain_key)
    }


    //This is the SCIMP style forward secrecy chain key iteration.
    fn derive_next_chain_and_message_key(a : &u64) -> (u64,u64) {
        let mut rng = get_rng(*a);
        let chain_key = rng.next_u64();
        let message_key = rng.next_u64();
        (chain_key,message_key)
    }


    fn encrypt_message(
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

    fn authenticate_message(_ : &AxolotlMessage<Substitution>, _ : &u64, _ : &u64, _ : &u64) {
    }

    fn ratchet_keys_are_equal(a : &u64, b : &u64) -> bool {
        *a == *b
    }

    fn generate_ratchet_key_pair() -> DHKeyPair<DHKey> {
        let key = next_rng().next_u64();
        let public = DHKey::public(&key);
        DHKeyPair{ key : key, public : public }
    }


    fn future_message_limit() -> u32 {
        1024
    }

    fn chain_message_limit() -> u32 {
        1024
    }


    fn skipped_chain_limit() -> usize {
        8
    }
}

pub fn init_alice_and_bob() -> (AxolotlState<Substitution>, AxolotlState<Substitution>) {
    let alice_identity = Substitution::generate_ratchet_key_pair();
    let alice_handshake = Substitution::generate_ratchet_key_pair();
    let bob_identity = Substitution::generate_ratchet_key_pair();
    let bob_handshake = Substitution::generate_ratchet_key_pair();
    let initial_ratchet = Substitution::generate_ratchet_key_pair();

    let alice_exchanged_identity = DHExchangedPair { mine : alice_identity.key, theirs : bob_identity.public };
    let alice_exchanged_handshake = DHExchangedPair { mine : alice_handshake.key, theirs : bob_handshake.public };
    let bob_exchanged_identity = DHExchangedPair { mine : bob_identity.key, theirs : alice_identity.public };
    let bob_exchanged_handshake = DHExchangedPair { mine : bob_handshake.key, theirs : alice_handshake.public };

    let alice = init_as_alice::<Substitution>(&alice_exchanged_identity, &alice_exchanged_handshake, &initial_ratchet.public);
    let bob = init_as_bob::<Substitution>(&bob_exchanged_identity, &bob_exchanged_handshake, initial_ratchet);

    (alice,bob)
}

pub fn check_send(sender : &mut AxolotlState<Substitution>, receiver : &mut AxolotlState<Substitution>, message : String) -> AxolotlMessage<Substitution> {
    let m = message.into_bytes();
    let encrypted = sender.encrypt(&m);
    let decrypted = receiver.decrypt(&encrypted.0, encrypted.1).unwrap();
    assert!(m[..] == decrypted[..]);
    assert!(m[..] != encrypted.0.ciphertext[..]);
    encrypted.0
}
