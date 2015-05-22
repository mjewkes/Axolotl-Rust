pub use axolotl::{self,Axolotl,AxolotlMessage,ExchangedPair,KeyPair};
use crypto_wrappers::{hkdf,hmac};
use std::ops::*;
use std::usize::MAX;


#[macro_export]
macro_rules! to_array(
    ($arr:expr, $count:expr) => ( {
        let mut x : [u8;$count] = [0; $count];
        for i in 0..$count {
            x[i] = $arr[i];
        }
        x
    });
);

macro_rules! derive_deref {
    ($id:ident) => (
        impl Deref for $id {
            type Target = [u8;32];

            fn deref<'a>(&'a self) -> &[u8;32]{
                &self.0
            }
        }
    )
}

#[derive(Clone)]
pub struct PrivateKey( [u8;KEY_LEN] );
derive_deref!(PrivateKey);

#[derive(Clone)]
pub struct PublicKey( [u8;KEY_LEN] );
derive_deref!(PublicKey);

#[derive(Clone)]
pub struct SharedKey( [u8;KEY_LEN] );
derive_deref!(SharedKey);

const KEY_LEN_CHAIN : usize = KEY_LEN;
const KEY_LEN_ROOT  : usize = KEY_LEN;
const KEY_LEN_MAC   : usize = KEY_LEN;
const KEY_LEN_CIPHER: usize = KEY_LEN;
const KEY_LEN_IV    : usize = 16;

const SEED_NULL     : [u8;1] = [0]; 
const SEED_MSG_KEY  : [u8;1] = [1];
const SEED_CHAIN_KEY: [u8;1] = [2];

const KEY_LEN : usize = 32;

pub type KeyGenFunc = fn( )                               -> ([u8;KEY_LEN],[u8;KEY_LEN]);
pub type PubKeyFunc = fn( [u8; KEY_LEN] )                 -> ([u8;KEY_LEN]); 
pub type DhFunc     = fn( [u8; KEY_LEN], [u8; KEY_LEN] )  -> ([u8;KEY_LEN]);

pub type EncryptFunc = fn( &[u8], [u8; KEY_LEN], [u8; KEY_LEN_IV] ) -> (Vec<u8>);
pub type DecryptFunc = fn( &[u8], [u8; KEY_LEN], [u8; KEY_LEN_IV] ) -> (Vec<u8>);

pub struct BaseImplementation{
   
    fn_key_gen :        KeyGenFunc,
    fn_key_gen_pub :    PubKeyFunc,             // To be removed
    fn_dhke :           DhFunc,

    kdf_info_init :     String,
    kdf_info_ratchet :  String,
    kdf_info_msg :      String,

    fn_enc :            EncryptFunc,
    fn_dec :            DecryptFunc,

    master_key_prefix : Option<[u8;KEY_LEN]>,   //TODO: Add Optional Prefix 
}

impl BaseImplementation {
    pub fn init(
        fn_priv_key_gen :   KeyGenFunc,
        fn_pub_key_gen :    PubKeyFunc,
        fn_dhke :           DhFunc,
        kdf_info_init :     String,
        kdf_info_ratchet :  String,
        kdf_info_msg :      String,
        fn_enc :            EncryptFunc,
        fn_dec :            DecryptFunc,
    ) -> Self{
        BaseImplementation{
            master_key_prefix : None, 
            fn_key_gen : fn_priv_key_gen,
            fn_key_gen_pub : fn_pub_key_gen,
            fn_dhke  :  fn_dhke,
            kdf_info_init : kdf_info_init,
            kdf_info_ratchet : kdf_info_ratchet,
            kdf_info_msg : kdf_info_msg,
            fn_enc : fn_enc,
            fn_dec : fn_dec,
        }
    } 
}

impl Axolotl for BaseImplementation{
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;
    type SharedSecret = SharedKey;

    type RootKey = RootKey;
    type ChainKey = ChainKey;
    type MessageKey = MessageKey;

    type PlainText = PlainText;
    type CipherText = CipherTextAndVersion;

    type Mac = hmac::MacResult;

    fn derive_initial_root_key_and_chain_key(
        &self,
        local_identity_remote_handshake_dh_secret : &Self::SharedSecret, 
        local_handshake_remote_identity_dh_secred : &Self::SharedSecret, 
        local_handshake_remote_handshake_dh_secret : &Self::SharedSecret
    ) -> (Self::RootKey, Self::ChainKey){
        
        let mut master_key : Vec<u8> = Vec::<u8>::new();

        match self.master_key_prefix{
            None => {},
            Some(val) => master_key.extend(val.iter().map(|&x| x)) // Can we remove and append Zero Bytes? 
        }
       
        master_key.extend(local_identity_remote_handshake_dh_secret.iter().map(|&x| x));
        master_key.extend(local_handshake_remote_identity_dh_secred.iter().map(|&x| x));
        master_key.extend(local_handshake_remote_handshake_dh_secret.iter().map(|&x| x));

        let (rk, ck) = keys_from_kdf(&master_key[..], self.kdf_info_init.as_bytes(),&SEED_NULL);
        (RootKey(rk),ChainKey(ck))
    }

    // This is the DH future secrecy ratchet/
    fn derive_next_root_key_and_chain_key(
        &self,
        RootKey(root_bytes) : Self::RootKey, 
        ratchet : &Self::SharedSecret
    ) -> (Self::RootKey, Self::ChainKey){
        let ikm = **ratchet;
        let (rk,ck) = keys_from_kdf(&ikm,self.kdf_info_ratchet.as_bytes(),&root_bytes);
        (RootKey(rk),ChainKey(ck))
    }

    //This is the SCIMP style forward secrecy chain key iteration.
    fn derive_next_chain_and_message_key(
        &self,
        chain_key : &Self::ChainKey
    ) -> (Self::ChainKey, Self::MessageKey){
        let ikm = chain_key.hmac( &SEED_MSG_KEY ); 
        let msg_key = generate_message_key(&ikm,self.kdf_info_msg.as_bytes(),&SEED_NULL);
        (chain_key.next(),msg_key)
    }

    fn encrypt_message(
        &self,
        message_key : &Self::MessageKey,
        plaintext : &Self::PlainText
    ) -> Self::CipherText{
        
        let PlainText(ref text) = *plaintext;
        let ciphertext = (self.fn_enc)(text,message_key.cipher_key, message_key.iv);
        CipherTextAndVersion {
            version : 3,
            cipher_text : ciphertext.into_boxed_slice(),
        }
    }

    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey,
        ciphertext : &Self::CipherText
    ) -> Option<Self::PlainText>{

        if ciphertext.version != 3 {
            return None;
        }

        let result = (self.fn_dec)(&ciphertext.cipher_text, message_key.cipher_key, message_key.iv);
        Some(PlainText(result.into_boxed_slice()))
    }

    fn authenticate_message(
        &self,
        message : &AxolotlMessage<Self>, 
        message_key : &Self::MessageKey, 
        sender_identity : &Self::PublicKey, 
        receiver_identity : &Self::PublicKey
    ) -> Self::Mac{
  
        let mut mac_state = hmac::HmacSha256::new(&message_key.mac_key);    // TODO: Need Discussion about if we want this to be configurable
        mac_state.input(&**sender_identity);                                // By calling out to a function with the MacKey+args or if we want 
        mac_state.input(&**receiver_identity);                              // to fix format/MAC
        mac_state.input(&message.ciphertext.cipher_text[..]); //TODO: input the version
        hmac::truncate_mac_result(mac_state.result(), 8)
    }

    fn ratchet_keys_are_equal(
        &self,
        key0 : &Self::PublicKey, 
        key1 : &Self::PublicKey
    ) -> bool{
        unimplemented!();
    }

    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>{
        let (private_bytes,public_bytes) = (self.fn_key_gen)();

        KeyPair{
            key: PrivateKey(private_bytes), 
            public: PublicKey(public_bytes)
        }

    }

    fn derive_shared_secret(&self, key : &Self::PrivateKey, public : &Self::PublicKey) -> Self::SharedSecret{
        SharedKey((self.fn_dhke)(**key,**public))
    }

    fn derive_public_key(&self, key : &Self::PrivateKey) -> Self::PublicKey{
        let public = (self.fn_key_gen_pub)(**key);
        PublicKey(public)
    }

    fn future_message_limit(&self) -> usize{
        2000
    }

    fn chain_message_limit(&self) -> usize{
        MAX
    }

    fn skipped_chain_limit(&self) -> usize{
        5
    }
}


#[derive(Clone)]
pub struct RootKey ([u8;32]);

#[derive(Clone)]
pub struct ChainKey ([u8;KEY_LEN_CHAIN]);

impl ChainKey {
    fn next(self : &Self) -> ChainKey {
        ChainKey(self.hmac(&SEED_CHAIN_KEY))
    }

    fn hmac(self : &Self, seed : &[u8] ) -> [u8;KEY_LEN_CHAIN] {  
        let ChainKey(key_bytes) = *self;  
        let mut hmac_context = hmac::HmacSha256::new(&key_bytes);
        hmac_context.input(seed);                           
        to_array!(hmac_context.result().code()[..],KEY_LEN_CHAIN)
    }
}

#[derive(Clone)]
pub struct MessageKey{
    cipher_key : [u8; KEY_LEN_CIPHER],
    mac_key : [u8; KEY_LEN_MAC],
    iv : [u8;16],
}

pub struct PlainText(pub Box<[u8]>);

impl PlainText {
    pub fn from_vec(data : Vec<u8>) -> PlainText {
        PlainText(data.into_boxed_slice())
    }
}

#[derive(Debug)]
pub struct CipherTextAndVersion{
    pub cipher_text : Box<[u8]>,
    version : u8,
}


/// Returns bytes of the dervived root and chain keys.
fn keys_from_kdf(input_key_material : &[u8] , info : &[u8], salt : &[u8]  ) -> ([u8; KEY_LEN_ROOT], [u8; KEY_LEN_CHAIN]) {
    let mut output_key_material :[u8; KEY_LEN_ROOT + KEY_LEN_CHAIN ] = [0; KEY_LEN_ROOT + KEY_LEN_CHAIN]; 
    hkdf::derive_key(input_key_material, info,salt,&mut output_key_material);
    split_raw_keys(output_key_material)
}

/// Derives the required keys and returns a newly allocated MessageKey
fn generate_message_key(input_key_material : &[u8] , info : &[u8], salt : &[u8]  ) -> MessageKey {
    const LEN : usize = KEY_LEN_CIPHER + KEY_LEN_MAC + KEY_LEN_IV;
    let mut output_key_material : [u8;LEN] = [0;LEN];     // WS: Why 96? 16 bytes are wasted

    hkdf::derive_key(input_key_material, info,salt,&mut output_key_material);
    let (cipher_key,mac_key,iv) = split_raw_msg_keys(output_key_material);

    MessageKey{ cipher_key: cipher_key , mac_key: mac_key, iv: iv}
}

/// Partitions an array literal into 2 dijoint array literals.
fn split_raw_keys(bytes: [u8; KEY_LEN_ROOT+KEY_LEN_CHAIN]) -> ([u8;KEY_LEN_ROOT], [u8;KEY_LEN_CHAIN]) {
    let root_key  : [u8; KEY_LEN_ROOT]  = to_array!( bytes[             .. KEY_LEN_ROOT                 ], KEY_LEN_ROOT );
    let chain_key : [u8; KEY_LEN_CHAIN] = to_array!( bytes[ KEY_LEN_ROOT.. KEY_LEN_ROOT + KEY_LEN_CHAIN ], KEY_LEN_CHAIN );
    (root_key,chain_key)
}

/// Partitions an array literal into 3 dijoint array literals corresponding to the CipherKey, 
/// MacKey, and IV. These three items are used to create a MessageKey.
fn split_raw_msg_keys(bytes: [u8; KEY_LEN_CIPHER+KEY_LEN_MAC+KEY_LEN_IV]) -> ([u8;KEY_LEN_CIPHER], [u8;KEY_LEN_MAC], [u8;KEY_LEN_IV]) {
    const MAC_OFFSET : usize = KEY_LEN_CIPHER+KEY_LEN_MAC;
    let cipher_key  : [u8; KEY_LEN_CIPHER] = to_array!( bytes[               ..KEY_LEN_CIPHER   ]   , KEY_LEN_CIPHER );
    let mac_key     : [u8; KEY_LEN_MAC]    = to_array!( bytes[ KEY_LEN_CIPHER..MAC_OFFSET       ]   , KEY_LEN_MAC );
    let iv          : [u8; KEY_LEN_IV]     = to_array!( bytes[     MAC_OFFSET..                 ]   , KEY_LEN_IV );

    (cipher_key,mac_key,iv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{KEY_LEN,KEY_LEN_CIPHER,KEY_LEN_IV};
    use crypto_wrappers::{aes_cbc,curve25519};

    // ========= PASS INS ===========
    pub fn dhkey_pair() -> KeyPair<BaseImplementation> {
        let (private,public)    = gen_keypair();
        let priv_key            = PrivateKey(private);
        let pub_key             = PublicKey(public);

        KeyPair{ key :priv_key, public : pub_key}
    } 

    pub fn gen_keypair() -> ([u8;32],[u8;32]) {
        let p = curve25519::generate_private_key();
        (*p, *curve25519::derive_public_key(&p))
    }

    pub fn gen_pub(p : [u8;32]) -> ([u8;32]) {
        *curve25519::derive_public_key(&curve25519::PrivateKey(p))
    }

    pub fn curve25519_shared(private : [u8;32], public :[u8;32]) -> [u8;32] {
        let Pk = curve25519::PrivateKey(private);
        let pk = curve25519::PublicKey(public);
        *curve25519::derive_shared_key(&Pk,&pk)
    }

    pub fn ext_encrypt(data : &[u8], key :  [u8; KEY_LEN_CIPHER], iv : [u8;KEY_LEN_IV]) -> Vec<u8>{    // Need to Change iv-len to allow for CTR or other modes
        aes_cbc::encrypt_aes256_cbc_mode(data,key, iv)
    }

    pub fn ext_decrypt(ciphertext : &[u8], key :  [u8; KEY_LEN_CIPHER], iv : [u8;KEY_LEN_IV]) -> Vec<u8>{
        aes_cbc::decrypt_aes256_cbc_mode(&ciphertext, key, iv)
    }

    #[test]
    fn dynamic_echo_roundtrip(){

        let base_impl = BaseImplementation::init(
            gen_keypair,
            gen_pub,
            curve25519_shared,

            "WhisperText".to_string(),
            "WhisperRatchet".to_string(),
            "WhisperMessageKeys".to_string(),

            ext_encrypt,
            ext_decrypt,
        );

        let alice_identity  = dhkey_pair();
        let alice_handshake = dhkey_pair();
        let bob_identity = dhkey_pair();
        let bob_handshake = dhkey_pair();
        let initial_ratchet = dhkey_pair();

        let alice_exchanged_identity = ExchangedPair::<BaseImplementation> { mine : alice_identity.key, theirs : bob_identity.public };
        let alice_exchanged_handshake = ExchangedPair::<BaseImplementation>  { mine : alice_handshake.key, theirs : bob_handshake.public };
        let bob_exchanged_identity = ExchangedPair::<BaseImplementation>  { mine : bob_identity.key, theirs : alice_identity.public };
        let bob_exchanged_handshake = ExchangedPair::<BaseImplementation>  { mine : bob_handshake.key, theirs : alice_handshake.public };

        let mut alice = axolotl::init_as_alice::<BaseImplementation>(&base_impl, &alice_exchanged_identity, &alice_exchanged_handshake, &initial_ratchet.public);
        let mut bob   = axolotl::init_as_bob::<BaseImplementation>(&base_impl, &bob_exchanged_identity, &bob_exchanged_handshake, initial_ratchet);

        let plaintext = PlainText::from_vec("Hello internet".to_string().into_bytes());
        let (msg, mac) = alice.encrypt(&base_impl,&plaintext);
        let reply = bob.decrypt(&base_impl, &msg,mac);

        match reply{
            None => panic!(),
            Some(r) => assert_eq!(plaintext.0 , r.0)
        }
    }
}