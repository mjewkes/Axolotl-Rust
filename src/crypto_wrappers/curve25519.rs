/// A wrapper for the Curve25519-Donna implementation 
/// 
/// TODO: Should be pulled into it's own project

use rand::{OsRng, Rng};

pub const PUB_KEY_LEN : usize = 32;
pub const PRIV_KEY_LEN : usize = 32;
pub const SHARED_KEY_LEN : usize = 32;

#[link(name = "curve25519-donna")]
extern {
    #[link_name(curve25519_donna)]  // TODO: Doesn't seem to work yet.
    fn curve25519_donna(output: *mut u8, a : *const u8, b : *const u8);

//     extern int  curve25519_sign(unsigned char* signature_out, /* 64 bytes */
//                      const unsigned char* curve25519_privkey,  32 bytes 
//                      const unsigned char* msg, const unsigned long msg_len,
//                      const unsigned char* random); /* 64 bytes */

}

fn invoke_curve25519_donna( a: &[u8;32],  b: &[u8;32]) -> [u8;32] {
    let mut out : [u8;32] = [0;32];
    unsafe {
        curve25519_donna(&mut out[0],&a[0],&b[0]);
    }
    out
}

pub fn derive_public_key( private_key: &PrivateKey) -> PublicKey {
    // PublicKey gen as outlined by https://code.google.com/p/curve25519-donna/
    let ref basepoint : [u8;32] = [ 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    PublicKey{val: invoke_curve25519_donna(&private_key.val,basepoint)}
}

pub fn derive_shared_key( private_key: &PrivateKey, remote_public_key : &PublicKey ) -> SharedKey {
    SharedKey{val: invoke_curve25519_donna(&private_key.val,remote_public_key.to_bytes())}
}

pub fn generate_private_key() -> PrivateKey{
    let mut rng = OsRng::new().unwrap();
    let ref mut private_key : [u8;PRIV_KEY_LEN] = [0;PRIV_KEY_LEN]; 
    rng.fill_bytes(private_key);

    // PrivateKey Generation as outlined by https://code.google.com/p/curve25519-donna/
    private_key[0] &= 248;
    private_key[31] &= 127;
    private_key[31] |= 64;

    PrivateKey::from_bytes(*private_key)
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct PublicKey {
    val: [u8;PUB_KEY_LEN],
}
impl PublicKey {
    pub fn from_bytes(bytes : [u8 ; PUB_KEY_LEN]) -> Self {
        PublicKey{val: bytes}
    }
    
    pub fn to_bytes(&self) -> &[u8;PUB_KEY_LEN] {
        &self.val
    }
}

#[derive(Clone)]
pub struct PrivateKey {
    val: [u8;PRIV_KEY_LEN],
}
impl PrivateKey {
    pub fn from_bytes(bytes : [u8 ; PRIV_KEY_LEN]) -> Self {
        PrivateKey{val: bytes}
    }
}

#[derive(Clone)]
pub struct SharedKey {
    val: [u8;SHARED_KEY_LEN],
}
impl SharedKey {
    pub fn to_bytes(&self) -> &[u8;SHARED_KEY_LEN] {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_kat(){

        let alice_public_bytes: [u8;PUB_KEY_LEN] =     [0x1b, 0xb7, 0x59, 0x66, 0xf2, 0xe9, 0x3a, 0x36, 0x91, 0xdf, 
                                                        0xff, 0x94, 0x2b, 0xb2, 0xa4, 0x66, 0xa1, 0xc0, 0x8b, 0x8d, 
                                                        0x78, 0xca, 0x3f, 0x4d, 0x6d, 0xf8, 0xb8, 0xbf, 0xa2, 0xe4, 
                                                        0xee, 0x28];

        let alice_private_bytes: [u8;PRIV_KEY_LEN] =   [0xc8, 0x06, 0x43, 0x9d, 0xc9, 0xd2, 0xc4, 0x76, 0xff, 0xed,
                                                        0x8f, 0x25, 0x80, 0xc0, 0x88, 0x8d, 0x58, 0xab, 0x40, 0x6b, 
                                                        0xf7, 0xae, 0x36, 0x98, 0x87, 0x90, 0x21, 0xb9, 0x6b, 0xb4, 
                                                        0xbf, 0x59];

        let bob_public_bytes: [u8;PUB_KEY_LEN] =       [0x65, 0x36, 0x14, 0x99, 0x3d, 0x2b, 0x15, 0xee, 0x9e, 0x5f,
                                                        0xd3, 0xd8, 0x6c, 0xe7, 0x19, 0xef, 0x4e, 0xc1, 0xda, 0xae, 
                                                        0x18, 0x86, 0xa8, 0x7b, 0x3f, 0x5f, 0xa9, 0x56, 0x5a, 0x27, 
                                                        0xa2, 0x2f];

        let bob_private_bytes: [u8;PRIV_KEY_LEN] =     [0xb0, 0x3b, 0x34, 0xc3, 0x3a, 0x1c, 0x44, 0xf2, 0x25, 0xb6, 
                                                        0x62, 0xd2, 0xbf, 0x48, 0x59, 0xb8, 0x13, 0x54, 0x11, 0xfa, 
                                                        0x7b, 0x03, 0x86, 0xd4, 0x5f, 0xb7, 0x5d, 0xc5, 0xb9, 0x1b, 
                                                        0x44, 0x66];
    
        let shared_bytes: [u8;SHARED_KEY_LEN] =        [0x32, 0x5f, 0x23, 0x93, 0x28, 0x94, 0x1c, 0xed, 0x6e, 0x67, 
                                                        0x3b, 0x86, 0xba, 0x41, 0x01, 0x74, 0x48, 0xe9, 0x9b, 0x64, 
                                                        0x9a, 0x9c, 0x38, 0x06, 0xc1, 0xdd, 0x7c, 0xa4, 0xc4, 0x77, 
                                                        0xe6, 0x29];

        let alice_pub  = PublicKey::from_bytes(alice_public_bytes);
        let alice_priv = PrivateKey::from_bytes(alice_private_bytes);

        let bob_pub  = PublicKey::from_bytes(bob_public_bytes);
        let bob_priv = PrivateKey::from_bytes(bob_private_bytes);

        let expected_shared = SharedKey{val:shared_bytes};

        let alice_shared = derive_shared_key(&alice_priv,&bob_pub);
        let bob_shared   = derive_shared_key(&bob_priv,&alice_pub);
        
        assert_eq!(alice_pub.to_bytes() , derive_public_key(&alice_priv).to_bytes());
        assert_eq!(bob_pub.to_bytes() , derive_public_key(&bob_priv).to_bytes());
        assert!(alice_pub.to_bytes() != derive_public_key(&bob_priv).to_bytes());

        assert_eq!(alice_shared.to_bytes(),bob_shared.to_bytes());
        assert_eq!(bob_shared.to_bytes(), expected_shared.to_bytes());
    }

    #[test]
    fn shared_secret_derivation(){
        let alice_priv = generate_private_key();
        let alice_pub = derive_public_key(&alice_priv);

        let bob_priv = generate_private_key();
        let bob_pub = derive_public_key(&bob_priv);

        let alice_shared = derive_shared_key(&alice_priv,&bob_pub);
        let bob_shared   = derive_shared_key(&bob_priv,&alice_pub);

        assert_eq!(alice_shared.to_bytes(), bob_shared.to_bytes());
    }

    fn to_pub(key: &SharedKey) -> PublicKey{
        PublicKey{val: *key.to_bytes()}
    }

    const LOOP_ITERATIONS : u32 = 10;
    #[test]
    fn loop_test(){
        let mut e1 = [0x03, 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let mut e2 = [0x05, 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let mut k  = [0x09, 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
                
        for __ in 0 .. LOOP_ITERATIONS {

            let e1k   = derive_shared_key(&PrivateKey::from_bytes(e1),&PublicKey::from_bytes(k));
            let e2e1k = derive_shared_key(&PrivateKey::from_bytes(e2),&to_pub(&e1k));
            let e2k   = derive_shared_key(&PrivateKey::from_bytes(e2),&PublicKey::from_bytes(k));
            let e1e2k = derive_shared_key(&PrivateKey::from_bytes(e1),&to_pub(&e2k));

            assert_eq!(e1e2k.to_bytes(),e2e1k.to_bytes());
            for x in 0..32 {
                e1[x] = e1[x] ^ e2k.to_bytes()[x];
                e2[x] = e2[x] ^ e1k.to_bytes()[x];
                k[x]  = k[x]  ^ e1e2k.to_bytes()[x];
            }
        }
    }
}