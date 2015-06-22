extern crate raxolotl;
extern crate serde;
use std::borrow::Borrow;
mod toy_stream_cipher;
use toy_stream_cipher::*;
use raxolotl::axolotl::*;

#[test]
fn alice_sends_to_bob() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
}   

#[test]
fn bob_sends_to_alice() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
}   

#[test]
fn alice_sends_then_bob_sends() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
}   

#[test]
fn bob_sends_then_alice_sends() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
}   

#[test]
fn alice_sends_many_to_bob() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    for i in 0..10 {
        let encrypted = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
        assert!(encrypted.message_number == i)
    }
}

#[test]
fn bob_sends_many_to_alice() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    for i in 0..10 {
        let encrypted = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
        assert!(encrypted.message_number == i)
    }
}

#[test]
fn interleave_alice_first() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    for i in 0..10 {
        for _ in 0..i {
            check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
        }
        for _ in 0..i {
            check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
        }
    }
}

#[test]
fn alice_ciphertext_not_repeated() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    let m0 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    let m1 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    assert!(m0.ciphertext[..] != m1.ciphertext[..]);
}

#[test]
fn bob_ciphertext_not_repeated() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    let m0 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    let m1 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    assert!(m0.ciphertext[..] != m1.ciphertext[..]);
}

#[test]
fn alice_ratchet_key_repeated() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    let m0 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    let m1 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    assert!(m0.ratchet_key == m1.ratchet_key);
    assert!(m0.message_number == 0);
    assert!(m1.message_number == 1);
}

#[test]
fn bob_ratchet_key_repeated() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    let m0 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    let m1 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    assert!(m0.ratchet_key == m1.ratchet_key);
    assert!(m0.message_number == 0);
    assert!(m1.message_number == 1);
}

#[test]
fn ratchet_key_not_repeated() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);
    let to_bob_0 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    let to_alice_0 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());
    let to_bob_1 = check_send(axolotl_impl, &mut alice, &mut bob, "hello, bob!".to_string());
    let to_alice_1 = check_send(axolotl_impl, &mut bob, &mut alice, "hello, alice!".to_string());

    assert!(to_bob_0.ratchet_key != to_bob_1.ratchet_key);
    assert!(to_alice_0.ratchet_key != to_alice_1.ratchet_key);
}

#[test]
fn serialize_idempotent() {
    let ref axolotl_impl = Substitution;
    let (alice, bob) = init_alice_and_bob(axolotl_impl);

    //copy via serialization
    let alice_json = serde::json::to_string(&alice).unwrap();
    let bob_json = serde::json::to_string(&bob).unwrap();
    let alice2 : AxolotlState<Substitution> = serde::json::from_str(alice_json.borrow()).unwrap();
    let bob2 : AxolotlState<Substitution>  = serde::json::from_str(bob_json.borrow()).unwrap();

    let alice2_json = serde::json::to_string(&alice2).unwrap();
    let bob2_json = serde::json::to_string(&bob2).unwrap();
    assert!(alice_json[..] == alice2_json[..]);
    assert!(bob_json[..] == bob2_json[..]);
}

#[test]
fn serialize_round_trip() {
    let ref axolotl_impl = Substitution;
    let (mut alice, mut bob) = init_alice_and_bob(axolotl_impl);

    //copy via serialization
    let alice_json = serde::json::to_string(&alice).unwrap();
    let bob_json = serde::json::to_string(&bob).unwrap();
    let mut alice2 = serde::json::from_str(alice_json.trim()).unwrap();
    let mut bob2   = serde::json::from_str(bob_json.trim()).unwrap();

    //check alice and bob2
    check_send(axolotl_impl, &mut alice, &mut bob2, "hello, bob2!".to_string());
    check_send(axolotl_impl, &mut bob2, &mut alice, "hello, alice!".to_string());

    //check alice2 and bob
    check_send(axolotl_impl, &mut alice2, &mut bob, "hello, bob!".to_string());
    check_send(axolotl_impl, &mut bob, &mut alice2, "hello, alice2!".to_string());
}
