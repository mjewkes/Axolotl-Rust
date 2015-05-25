extern crate raxolotl;
pub use self::raxolotl::axolotl::{Axolotl,KeyPair};

use whisper_protocol::crypto_wrappers::{aes_cbc,curve25519,hkdf,hmac};


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


const KEY_LEN_CHAIN : usize = 32;
const KEY_LEN_ROOT  : usize = 32;
const KEY_LEN_MAC   : usize = 32;
const KEY_LEN_CIPHER: usize = 32;
const KEY_LEN_IV    : usize = 16;

const SEED_NULL     : [u8;1] = [0]; 
const SEED_MSG_KEY  : [u8;1] = [1];
const SEED_CHAIN_KEY: [u8;1] = [2];

pub struct TextSecureV3;



#[derive(Clone)]
pub struct Rootkey ([u8;32]);

#[derive(Clone)]
pub struct ChainKey ([u8;KEY_LEN_CHAIN]);          // Should this have an indexVariable?

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
    cipher_key : [u8;32],
    mac_key : [u8;32],
    iv : [u8;16],
}

pub struct PlainText(pub Box<[u8]>);

impl PlainText {
    pub fn from_vec(data : Vec<u8>) -> PlainText {
        PlainText(data.into_boxed_slice())
    }
}

pub struct CipherTextAndVersion{
    pub cipher_text : Box<[u8]>,
    version : u8,
}

pub struct Message {
    pub message_number : usize,
    pub ratchet_key : curve25519::PublicKey,
    pub ciphertext : CipherTextAndVersion,   
}

impl Axolotl for TextSecureV3{
    type PrivateKey = curve25519::PrivateKey;
    type PublicKey  = curve25519::PublicKey;
    type SharedSecret  = curve25519::SharedKey;

    type RootKey = Rootkey;
    type ChainKey = ChainKey;
    type MessageKey = MessageKey;

    type PlainText = PlainText;
    type CipherText = CipherTextAndVersion;

    type Mac = hmac::MacResult;

    type Message = Message; //TODO: replace with protocol buffer

    /// Returns initial Root and Chain keys derived from initial the TripleDH handshake. 
    fn derive_initial_root_key_and_chain_key(
        &self,
        local_identity_remote_handshake_dh_secret : &Self::SharedSecret, 
        local_handshake_remote_identity_dh_secred : &Self::SharedSecret, 
        local_handshake_remote_handshake_dh_secret : &Self::SharedSecret
    ) -> (Self::RootKey, Self::ChainKey){

        let disconuity_bytes = curve25519::SharedKey::from_bytes([0xFF;32]);
        let mut master_key : Vec<u8> = [ &disconuity_bytes, local_identity_remote_handshake_dh_secret,
                            local_handshake_remote_identity_dh_secred,
                            local_handshake_remote_handshake_dh_secret]
                            .iter()
                            .flat_map(|x| {x.to_bytes()})
                            .map(|x|{*x})
                            .collect();

        let (rk, ck) = keys_from_kdf(&master_key[..], "WhisperText".as_bytes(),&SEED_NULL);
        (Rootkey(rk),ChainKey(ck))
    }

    /// Returns new Root and Chain keys derived from racheting previous keyset.
    fn derive_next_root_key_and_chain_key(
        &self, 
        Rootkey( root_bytes ): Self::RootKey, 
        ratchet : &Self::SharedSecret
    ) -> (Self::RootKey, Self::ChainKey) {
        let ikm = ratchet;
        let (rk,ck) = keys_from_kdf(ikm.to_bytes(),"WhisperRatchet".as_bytes(),&root_bytes);
        (Rootkey(rk),ChainKey(ck))
    }

    /// Returns derived Message key for given Chain key as well as the next Chain key to be used.
    fn derive_next_chain_and_message_key(&self, chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){
        let ikm = chain_key.hmac( &SEED_MSG_KEY ); 
        let msg_key = generate_message_key(&ikm,"WhisperMessageKeys".as_bytes(),&SEED_NULL);
        (chain_key.next(),msg_key)
    }
    
    fn encrypt_message(
        &self,
        message_key : &Self::MessageKey, 
        plaintext : &Self::PlainText
    ) -> Self::CipherText{

        let PlainText(ref text) = *plaintext;
        let ciphertext = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);
        
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

        let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext.cipher_text, message_key.cipher_key, message_key.iv);
        Some(PlainText(result.into_boxed_slice()))
    }

    fn authenticate_message(
        &self,
        message : &Self::Message,
        message_key : &Self::MessageKey, 
        sender_identity : &Self::PublicKey, 
        receiver_identity : &Self::PublicKey
    ) -> Self::Mac{

        let mut mac_state = hmac::HmacSha256::new(&message_key.mac_key);
        mac_state.input(sender_identity.to_bytes());
        mac_state.input(receiver_identity.to_bytes());
        mac_state.input(&message.ciphertext.cipher_text[..]); //TODO: input the version
        hmac::truncate_mac_result(mac_state.result(), 8)
    }

    fn encode_header_and_ciphertext(
        &self,
        header : (usize, Self::PublicKey),
        ciphertext : Self::CipherText
    ) -> Self::Message {
        Message{
            message_number : header.0,
            ratchet_key : header.1,
            ciphertext : ciphertext,
        }
    }

    fn with_decoded_header<F,T>(&self, message : &Self::Message, f : F
    ) -> T where F:FnOnce(usize, &Self::PublicKey) -> T {
        f(message.message_number, &message.ratchet_key)
    }

    fn with_decoded_ciphertext<F,T>(&self, message : &Self::Message, f : F
    ) -> T where F:FnOnce(&Self::CipherText) -> T {
        f(&message.ciphertext)
    }

    fn ratchet_keys_are_equal(&self, key0 : &Self::PublicKey, key1 : &Self::PublicKey) -> bool{
        key0 == key1
    }
    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>{
        let priv_key  = curve25519::generate_private_key();
        let pub_key = curve25519::derive_public_key(&priv_key);

        KeyPair{ key: priv_key, public : pub_key }
    }

    fn derive_public_key(&self, key : &Self::PrivateKey) -> Self::PublicKey {
        curve25519::derive_public_key(key)
    }
    fn derive_shared_secret(&self, mine : &Self::PrivateKey, theirs : &Self::PublicKey) -> Self::SharedSecret {
        curve25519::derive_shared_key( mine,theirs)
    }

    fn future_message_limit(&self) -> usize{
        2000
    }
    fn chain_message_limit(&self) -> usize
    {
        usize::max_value()
    }

    fn skipped_chain_limit(&self) -> usize{
        5
    }
}

/// Returns bytes of the dervived root and chain keys.
fn keys_from_kdf(input_key_material : &[u8] , info : &[u8], salt : &[u8]  ) -> ([u8;KEY_LEN_ROOT], [u8;KEY_LEN_CHAIN]) {
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
