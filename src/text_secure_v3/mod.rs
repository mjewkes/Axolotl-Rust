use ::axolotl;
use ::axolotl::{AxolotlMessage,DH,DHKeyPair,DHPublic,DHShared};
use ::crypto_wrappers::aes_cbc;
use ::crypto_wrappers::hkdf;
use ::crypto_wrappers::hmac;


macro_rules! to_array(
    ($arr:expr, $count:expr) => ( {
        let mut x : [u8;$count] = [0; $count];
        for i in 0..$count {
            x[i] = $arr[i];
        }
        x
    });
);

const HMAC_LEN      : usize = 32;

const KEY_LEN_CHAIN : usize = 32;
const KEY_LEN_ROOT  : usize = 32;
const KEY_LEN_MAC   : usize = 32;
const KEY_LEN_CIPHER: usize = 32;
const KEY_LEN_IV    : usize = 16;

const SEED_NULL     : [u8;1] = [0]; 
const SEED_MSG_KEY  : [u8;1] = [1];
const SEED_CHAIN_KEY: [u8;1] = [2];

pub struct TextSecureV3{
    my_identity_key : axolotl::DHPublic<IdentityKey>,
    their_identity_key : axolotl::DHPublic<IdentityKey>,
}
impl TextSecureV3{
    pub fn new(my_identity_key : axolotl::DHPublic<IdentityKey>, 
               their_identity_key : axolotl::DHPublic<IdentityKey>,) 
               -> TextSecureV3{
        TextSecureV3{my_identity_key : my_identity_key, their_identity_key: their_identity_key}
    } 
    fn match_mac_from_keys_and_bytes_to_truncated_mac (serialized_message_bytes : &[u8],
                                          mac_key : [u8;32],
                                          sender_public_key : &axolotl::DHPublic<IdentityKey>,
                                          receiver_public_key : &axolotl::DHPublic<IdentityKey>,
                                          truncated_mac : [u8;8])
                                          -> bool{
        let mut mac_context= hmac::HmacSha256::new(&mac_key);
        mac_context.input(sender_public_key);
        mac_context.input(receiver_public_key);
        mac_context.input(serialized_message_bytes);
        let mac_result = mac_context.result();
        let bytes = &mac_result.code()[0..8];
        bytes == truncated_mac
    }

}
pub struct IdentityKey;

impl axolotl::DH for IdentityKey {
    type Private = [u8;32];
    type Public = [u8;32];
    type Shared = [u8;32];

    fn public(key : &Self::Private) -> Self::Public{
        unimplemented!();
    }
    fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
        unimplemented!();
    }
}

pub struct RatchetKey;
impl axolotl::DH for RatchetKey {
    type Private = [u8;32];
    type Public = [u8;32];
    type Shared = [u8;32];

    fn public(key : &Self::Private) -> Self::Public{
        unimplemented!();
    }
    fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
        unimplemented!();
    }

}

#[derive(Clone)]
pub struct Rootkey ([u8;32]);

#[derive(Clone)]
pub struct ChainKey ([u8;KEY_LEN_CHAIN]);          // Should this have an indexVariable?

impl ChainKey {
    fn next(self : &Self) -> ChainKey {
        let ChainKey(key_bytes) = *self;  
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

pub struct PlainText(Box<[u8]>);

pub struct CipherTextAndVersion{
    cipher_text : Box<[u8]>,
    version : u8,
}

impl axolotl::Axolotl for TextSecureV3{
    type IdentityKey = IdentityKey;
    type RatchetKey = RatchetKey;

    type RootKey = Rootkey;
    type ChainKey = ChainKey;
    type MessageKey = MessageKey;

    type PlainText = PlainText;
    type CipherText = CipherTextAndVersion;

    type Mac = ();

    /// Returns initial Root and Chain keys derived from initial the TripleDH handshake. 
    fn derive_initial_root_key_and_chain_key(
        local_identity_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_identity_dh_secred : &DHShared<Self::IdentityKey>, 
        local_handshake_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>) -> (Self::RootKey, Self::ChainKey){
        
        let master_key = [  *local_identity_remote_handshake_dh_secret,
                            *local_handshake_remote_identity_dh_secred,
                            *local_handshake_remote_handshake_dh_secret].concat();

        let (rk, ck) = keys_from_kdf(&master_key[..], "WhisperText".as_bytes(),&SEED_NULL);
        (Rootkey(rk),ChainKey(ck))
    }

    /// Returns new Root and Chain keys derived from racheting previous keyset.
    fn derive_next_root_key_and_chain_key(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
        let Rootkey( root_bytes ) = root_key;
        let ikm = [root_bytes,*ratchet].concat();

        let (rk,ck) = keys_from_kdf(&ikm,"WhisperRatchet".as_bytes(),&root_bytes);

        (Rootkey(rk),ChainKey(ck))
    }

    /// Returns derived Message key for given Chain key as well as the next Chain key to be used.
    fn derive_next_chain_and_message_key(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){
 
        let ikm = chain_key.hmac( &SEED_MSG_KEY ); 
        let msg_key = generate_message_key(&ikm,"WhisperMessage".as_bytes(),&SEED_NULL);
        (chain_key.next(),msg_key)
    }
    
    fn encrypt_message(message_key : &Self::MessageKey, 
                      plaintext : &Self::PlainText) 
                      -> Self::CipherText{

        let PlainText(ref text) = *plaintext;
        let ciphertext = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);
        
        CipherTextAndVersion {
            version : 3,
            cipher_text : ciphertext.into_boxed_slice(),
        }
    }

    fn decrypt_message(message_key : &Self::MessageKey, 
                      ciphertext : &Self::CipherText) 
                      -> Option<Self::PlainText>{
        if ciphertext.version != 3 {
            return None;
        }

        let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext.cipher_text, message_key.cipher_key, message_key.iv);
        Some(PlainText(result.into_boxed_slice()))
    }

    fn authenticate_message(
        message : &AxolotlMessage<Self>, 
        message_key : &Self::MessageKey, 
        sender_identity : &DHPublic<Self::IdentityKey>, 
        receiver_identity : &DHPublic<Self::IdentityKey>) -> Self::Mac{

        unimplemented!();
    }

    fn ratchet_keys_are_equal(key0 : &<Self::RatchetKey as DH>::Public, key1 : &<Self::RatchetKey as DH>::Public) -> bool{
        unimplemented!();
    }
    fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey>{
        unimplemented!();
    }

    fn future_message_limit() -> u32{
        unimplemented!();
    }
    fn chain_message_limit() -> u32
    {
        2000
    }

    fn skipped_chain_limit() -> usize{
        unimplemented!();
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
    let root_key  : [u8; KEY_LEN_ROOT]  = to_array!( bytes[             ..KEY_LEN_ROOT  ], KEY_LEN_ROOT );
    let chain_key : [u8; KEY_LEN_CHAIN] = to_array!( bytes[ KEY_LEN_ROOT..KEY_LEN_CHAIN ], KEY_LEN_CHAIN );

    (root_key,chain_key)
}

/// Partitions an array literal into 3 dijoint array literals.
fn split_raw_msg_keys(bytes: [u8; KEY_LEN_CIPHER+KEY_LEN_MAC+KEY_LEN_IV]) -> ([u8;KEY_LEN_CIPHER], [u8;KEY_LEN_MAC], [u8;KEY_LEN_IV]) {
    let cipher_key  : [u8; KEY_LEN_CIPHER] = to_array!( bytes[                           ..KEY_LEN_CIPHER]  , KEY_LEN_CIPHER );
    let mac_key     : [u8; KEY_LEN_MAC]    = to_array!( bytes[             KEY_LEN_CIPHER..KEY_LEN_MAC]     , KEY_LEN_MAC );
    let iv          : [u8; KEY_LEN_IV]     = to_array!( bytes[ KEY_LEN_CIPHER+KEY_LEN_MAC..  ]              , KEY_LEN_IV );

    (cipher_key,mac_key,iv)
}

