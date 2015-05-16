extern crate raxolotl;

use raxolotl::text_secure_v3::{self, IdentityKey, TextSecureV3, PlainText};
use raxolotl::axolotl::{self, DH, DHKeyPair, DHExchangedPair};
use raxolotl::crypto_wrappers::curve25519;

// #[test]
// fn static_roundtrip_echo() {
//     unimplemented!();
// }

#[test]
fn dynamic_roundtrip_echo(){

    let (mut alice, mut bob ) = init_dynamic_axolotl_states();

    let msg : PlainText = b"hello goat!".to_vec().into();

    for __ in 0..10 {
        let (wm, mac) = alice.encrypt(&msg);
        let plaintext = bob.decrypt(&wm,mac).unwrap();

        assert_eq!(msg.0 , plaintext.0);

        let (wmb, macb) = bob.encrypt(&plaintext);
        let reply = alice.decrypt(&wmb,macb).unwrap();

        assert_eq!(msg.0,reply.0);
    }
}

fn dhkey_pair() -> DHKeyPair<IdentityKey> {
    let priv_key  = curve25519::generate_private_key();
    let pub_key = curve25519::derive_public_key(&priv_key);

    DHKeyPair{ key :priv_key, public : pub_key}
}


fn init_dynamic_axolotl_states() -> (axolotl::AxolotlState<TextSecureV3>, axolotl::AxolotlState<TextSecureV3>) {

    let alice_identity = dhkey_pair();
    let alice_handshake = dhkey_pair();
    let bob_identity = dhkey_pair();
    let bob_handshake = dhkey_pair();
    let initial_ratchet = text_secure_v3::ident_to_ratchet(dhkey_pair());

    let alice_exchanged_identity = DHExchangedPair { mine : alice_identity.key, theirs : bob_identity.public };
    let alice_exchanged_handshake = DHExchangedPair { mine : alice_handshake.key, theirs : bob_handshake.public };
    let bob_exchanged_identity = DHExchangedPair { mine : bob_identity.key, theirs : alice_identity.public };
    let bob_exchanged_handshake = DHExchangedPair { mine : bob_handshake.key, theirs : alice_handshake.public };

    let alice = axolotl::init_as_alice::<TextSecureV3>(&alice_exchanged_identity, &alice_exchanged_handshake, &initial_ratchet.public);
    let bob = axolotl::init_as_bob::<TextSecureV3>(&bob_exchanged_identity, &bob_exchanged_handshake, initial_ratchet);
    (alice,bob)
}
