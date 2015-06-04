mod whisper_protocol;

use whisper_protocol::{axolotl};
use whisper_protocol::crypto_wrappers::curve25519;
use whisper_protocol::text_secure_v3::{KeyPair,PlainText,TextSecureV3};

#[test]
fn dynamic_roundtrip_echo(){
    let ref axolotl_impl = TextSecureV3;
    let (mut alice, mut bob ) = init_dynamic_axolotl_states(axolotl_impl);

    let msg = PlainText::from_vec("hello goat!".to_string().into_bytes());

    for __ in 0..10 {
        let (wm, mac) = alice.encrypt(axolotl_impl, msg.clone());
        let plaintext = bob.decrypt(axolotl_impl, wm,mac).ok().unwrap();

        assert_eq!(msg.0 , plaintext.0);

        let (wmb, macb) = bob.encrypt(axolotl_impl, plaintext);
        let reply = alice.decrypt(axolotl_impl, wmb,macb).ok().unwrap();

        assert_eq!(msg.0,reply.0);
    }
}

// Test Vectors used are taken from libaxolotl to test cross implementation compatability
// (https://github.com/WhisperSystems/libaxolotl-java/blob/ef36fe43d5efcf75950779d0971e9e0e02730172/tests/src/test/java/org/whispersystems/libaxolotl/ratchet/RatchetingSessionTest.java)

#[test]
fn android_session_kat () {
    let ref axolotl_impl = TextSecureV3;
    let alice_identity_private_key: [u8;32]  =       [0x58, 0x20, 0xD9, 0x2B, 0xBF, 0x3E, 0x74, 0x80, 0x68, 0x01, 
                                                      0x94, 0x90, 0xC3, 0xAA, 0x94, 0x50, 0x21, 0xFA, 0xA6, 0xD2, 
                                                      0x43, 0xE4, 0x86, 0x49, 0xF6, 0x6B, 0xD6, 0xA4, 0x45, 0x99, 
                                                      0x17, 0x63];

    let alice_identity_public_key: [u8;32]  =        [0x6F, 0xEC, 0xDE, 0xE2, 0x7F, 0x67, 0x36, 0xA7, 0xC6, 0xA2, 
                                                      0x77, 0x9C, 0x5A, 0xDC, 0x26, 0x35, 0x22, 0x2A, 0xBB, 0x26, 
                                                      0x01, 0xCB, 0x93, 0x7B, 0xA9, 0x0F, 0xD8, 0x6E, 0x56, 0x2C, 
                                                      0x76, 0x1C];

    let alice_base_public_key: [u8;32]  =            [0x7B, 0xB2, 0x6A, 0xAF, 0x25, 0x3C, 0x7C, 0x2F, 0xFB, 0x99, 
                                                      0x42, 0xED, 0x5F, 0xDA, 0x93, 0x77, 0xF5, 0xD2, 0x3E, 0x25, 
                                                      0x95, 0x67, 0x5D, 0x62, 0x14, 0xBB, 0x3B, 0x40, 0xC7, 0xBE, 
                                                      0xAC, 0x56];

    let alice_base_private_key: [u8;32]  =           [0x28, 0xD3, 0x04, 0xA2, 0xEB, 0x00, 0xFB, 0x63, 0xF8, 0x5E, 
                                                      0x6D, 0x4C, 0xEF, 0xC6, 0xBF, 0x13, 0x1B, 0x5E, 0xE5, 0x62, 
                                                      0xB4, 0x6B, 0xD5, 0x2C, 0xCB, 0x52, 0x8A, 0x84, 0x61, 0xDD, 
                                                      0xC3, 0x65];

    let bob_identity_public_key: [u8;32]  =          [0x01, 0x6A, 0x60, 0xFC, 0xCF, 0x33, 0xB6, 0xF0, 0x9A, 0x1E, 
                                                      0x9B, 0x54, 0x77, 0x78, 0x42, 0xDD, 0xE6, 0xC4, 0xF6, 0x30, 
                                                      0xAE, 0x35, 0x95, 0x67, 0xB3, 0x74, 0x20, 0xCF, 0x2D, 0x93, 
                                                      0xF1, 0x45];

    let bob_identity_private_key: [u8;32]  =         [0xC8, 0xF3, 0xA6, 0x39, 0x34, 0xCE, 0xDE, 0xEE, 0x37, 0x07, 
                                                      0xFF, 0x79, 0x71, 0x05, 0x0D, 0x58, 0x3B, 0x63, 0x7D, 0xD2, 
                                                      0x21, 0x15, 0xE3, 0xFD, 0x2B, 0x1D, 0x41, 0x22, 0x2C, 0x29, 
                                                      0x24, 0x65];

    let bob_base_private_key: [u8;32]  =             [0x70, 0xCC, 0x77, 0x0A, 0x82, 0x74, 0x70, 0x99, 0xB7, 0xCC, 
                                                      0x05, 0xCC, 0x69, 0x73, 0x58, 0x78, 0x41, 0x3E, 0xCF, 0xEE, 
                                                      0xFE, 0x85, 0xB5, 0xF7, 0x14, 0xFF, 0x85, 0x36, 0x8C, 0x98, 
                                                      0x70, 0x52];

    let bob_base_public_key: [u8;32]  =              [0x18, 0x3A, 0x6E, 0xC2, 0xC7, 0x4A, 0x21, 0xF3, 0xDE, 0xB3, 
                                                      0x70, 0x4C, 0x3D, 0x32, 0x45, 0xE0, 0xA5, 0xD5, 0x5F, 0xDC, 
                                                      0xC9, 0x9A, 0x26, 0x9D, 0x64, 0x68, 0xA6, 0x7C, 0xAE, 0xEF, 
                                                      0x59, 0x12];

    let alice_sending_ratchet_private: [u8;32]  =    [0x98, 0x04, 0x0B, 0xAE, 0x6B, 0x3D, 0x02, 0x9C, 0xF1, 0x25, 
                                                      0xDC, 0x8E, 0xD8, 0x07, 0xCE, 0x33, 0xFC, 0xE0, 0x07, 0xD8, 
                                                      0x2F, 0x67, 0x6D, 0x7B, 0xC7, 0x1A, 0x5B, 0x91, 0x3B, 0x60, 
                                                      0x3B, 0x67];

    let alice_sending_ratchet_public: [u8;32]  =     [0xB6, 0x2A, 0xE0, 0x25, 0xB8, 0xFF, 0xEE, 0x3A, 0xEB, 0x01, 
                                                      0x1B, 0xF7, 0x78, 0xE6, 0x26, 0x22, 0x56, 0x17, 0x30, 0x7A, 
                                                      0x95, 0x87, 0x91, 0x31, 0xD9, 0x9D, 0x27, 0x49, 0x06, 0xEE, 
                                                      0x57, 0x6A];

    let alice_plaintext: [u8;28]  =                  [0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 
                                                      0x70, 0x6C, 0x61, 0x69, 0x6E, 0x74, 0x65, 0x78, 0x74, 0x20, 
                                                      0x6D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2E];

    let alice_cipher_text: [u8;32]  =                [0x9E, 0xF2, 0xD0, 0xE1, 0x30, 0x4C, 0x01, 0xE0, 0x68, 0x7B, 
                                                      0x44, 0x5A, 0x27, 0x64, 0x79, 0x51, 0xD4, 0xC7, 0x0B, 0xF3, 
                                                      0xD3, 0xAC, 0x23, 0xA5, 0x8D, 0xF7, 0x22, 0xDC, 0x22, 0x76, 
                                                      0xC3, 0xA6];

    
    let alice_identity_keypair  = dhkey_pair_from_bytes(alice_identity_private_key,alice_identity_public_key);
    let bob_identity_keypair    = dhkey_pair_from_bytes(bob_identity_private_key,bob_identity_public_key);
    
    let alice_base_keypair      = dhkey_pair_from_bytes(alice_base_private_key,alice_base_public_key);
    let bob_base_keypair        = dhkey_pair_from_bytes(bob_base_private_key,bob_base_public_key);

    let alice_sending_ratchet_keypair : KeyPair<TextSecureV3> = KeyPair{ 
                                            key : curve25519::PrivateKey::from_bytes(alice_sending_ratchet_private), 
                                            public : curve25519::PublicKey::from_bytes(alice_sending_ratchet_public)
                                        }; 
    let bob_ratchet_keypair = bob_base_keypair.clone();

    let mut alice = axolotl::init_as_alice_with_explicit_ratchet_keypair::<TextSecureV3>(
        axolotl_impl,
        &alice_identity_keypair.key,
        &bob_identity_keypair.public,
        &alice_base_keypair.key,
        &bob_base_keypair.public,
        alice_sending_ratchet_keypair,
        &bob_ratchet_keypair.public
    );

    let mut bob = axolotl::init_as_bob::<TextSecureV3>(
        axolotl_impl,
        &bob_identity_keypair.key,
        &alice_identity_keypair.public,
        &bob_base_keypair.key,
        &alice_base_keypair.public,
        bob_ratchet_keypair
    );
    

    assert_eq!(&b"This is a plaintext message."[..],&alice_plaintext[..]); //Encoding Check
    let a_plain = PlainText::from_vec(alice_plaintext.to_vec());


    let (alice_cipher_msg,ab_mac) = alice.encrypt(axolotl_impl, a_plain);
    assert_eq!(&alice_cipher_msg.ciphertext.cipher_text[..], &alice_cipher_text[..]);

   
    let bob_plain = bob.decrypt(axolotl_impl, alice_cipher_msg,ab_mac).ok().unwrap();
    assert_eq!(bob_plain.0.to_vec(),&alice_plaintext[..]);
   
    for i in 0 .. 100{
        let message = [i;78];

        let (c,m) = alice.encrypt(axolotl_impl, PlainText::from_vec(message.to_vec()));      
        assert_eq!(&message[..], &bob.decrypt(axolotl_impl, c,m).ok().unwrap().0.to_vec()[..] );
    }

    for i in 0 .. 100{
        let message = [i;1802];

        let (c,m) = bob.encrypt(axolotl_impl, PlainText::from_vec(message.to_vec()));
        assert_eq!(&message[..], &alice.decrypt(axolotl_impl, c,m).ok().unwrap().0.to_vec()[..] );
    }
}

// ------ Utitlities ------
fn dhkey_pair() -> KeyPair<TextSecureV3> {
    let priv_key  = curve25519::generate_private_key();
    let pub_key = curve25519::derive_public_key(&priv_key);

    KeyPair{ key :priv_key, public : pub_key}
} 

fn dhkey_pair_from_bytes(private : [u8;32], public: [u8;32]) -> KeyPair<TextSecureV3> {

    KeyPair{ key :curve25519::PrivateKey::from_bytes(private), public : curve25519::PublicKey::from_bytes(public)}
} 

fn init_dynamic_axolotl_states(axolotl_impl : &TextSecureV3) -> (axolotl::AxolotlState<TextSecureV3>, axolotl::AxolotlState<TextSecureV3>) {

    let alice_identity = dhkey_pair();
    let alice_handshake = dhkey_pair();
    let bob_identity = dhkey_pair();
    let bob_handshake = dhkey_pair();
    let initial_ratchet = dhkey_pair();

    let alice = axolotl::init_as_alice::<TextSecureV3>(
      axolotl_impl, 
      &alice_identity.key,
      &bob_identity.public,
      &alice_handshake.key,
      &bob_handshake.public, 
      &initial_ratchet.public
    );
    let bob = axolotl::init_as_bob::<TextSecureV3>(
      axolotl_impl, 
      &bob_identity.key,
      &alice_identity.public,
      &bob_handshake.key,
      &alice_handshake.public, 
      initial_ratchet
    );
    (alice,bob)
}