extern crate rand;
extern crate raxolotl;

mod toy_stream_cipher;
use toy_stream_cipher::*;

#[test]
fn alice_sends_to_bob() {
    let (mut alice, mut bob) = init_alice_and_bob();
    check_send(&mut alice, &mut bob, "hello, bob!".to_string());
}

#[test]
fn bob_sends_to_alice() {
    let (mut alice, mut bob) = init_alice_and_bob();
    check_send(&mut bob, &mut alice, "hello, alice!".to_string());
}

#[test]
fn alice_sends_then_bob_sends() {
    let (mut alice, mut bob) = init_alice_and_bob();
    check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    check_send(&mut bob, &mut alice, "hello, alice!".to_string());
}

#[test]
fn bob_sends_then_alice_sends() {
    let (mut alice, mut bob) = init_alice_and_bob();
    check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    check_send(&mut alice, &mut bob, "hello, bob!".to_string());
}

#[test]
fn alice_sends_many_to_bob() {
    let (mut alice, mut bob) = init_alice_and_bob();
    for i in 0..10 {
        let encrypted = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
        assert!(encrypted.message_number == i)
    }
}

#[test]
fn bob_sends_many_to_alice() {
    let (mut alice, mut bob) = init_alice_and_bob();
    for i in 0..10 {
        let encrypted = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
        assert!(encrypted.message_number == i)
    }
}

#[test]
fn interleave_alice_first() {
    let (mut alice, mut bob) = init_alice_and_bob();
    for i in 0..10 {
        for _ in 0..i {
            check_send(&mut alice, &mut bob, "hello, bob!".to_string());
        }
        for _ in 0..i {
            check_send(&mut bob, &mut alice, "hello, alice!".to_string());
        }
    }
}

#[test]
fn alice_ciphertext_not_repeated() {
    let (mut alice, mut bob) = init_alice_and_bob();
    let m0 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    let m1 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    assert!(m0.ciphertext[..] != m1.ciphertext[..]);
}

#[test]
fn bob_ciphertext_not_repeated() {
    let (mut alice, mut bob) = init_alice_and_bob();
    let m0 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    let m1 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    assert!(m0.ciphertext[..] != m1.ciphertext[..]);
}

#[test]
fn alice_ratchet_key_repeated() {
    let (mut alice, mut bob) = init_alice_and_bob();
    let m0 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    let m1 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    assert!(m0.ratchet_key == m1.ratchet_key);
    assert!(m0.message_number == 0);
    assert!(m1.message_number == 1);
}

#[test]
fn bob_ratchet_key_repeated() {
    let (mut alice, mut bob) = init_alice_and_bob();
    let m0 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    let m1 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    assert!(m0.ratchet_key == m1.ratchet_key);
    assert!(m0.message_number == 0);
    assert!(m1.message_number == 1);
}

#[test]
fn ratchet_key_not_repeated() {
    let (mut alice, mut bob) = init_alice_and_bob();
    let to_bob_0 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    let to_alice_0 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());
    let to_bob_1 = check_send(&mut alice, &mut bob, "hello, bob!".to_string());
    let to_alice_1 = check_send(&mut bob, &mut alice, "hello, alice!".to_string());

    assert!(to_bob_0.ratchet_key != to_bob_1.ratchet_key);
    assert!(to_alice_0.ratchet_key != to_alice_1.ratchet_key);
}
