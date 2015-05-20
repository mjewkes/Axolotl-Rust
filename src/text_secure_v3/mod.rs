use axolotl::{self, AxolotlMessage, DH, DHKeyPair, DHPublic, DHShared};
use crypto_wrappers::{aes_cbc, curve25519, hkdf, hmac};
use std::u32;

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

const WHOLE_BUNCH   : u32 = u32::MAX;

const KEY_LEN_CHAIN : usize = 32;
const KEY_LEN_ROOT  : usize = 32;
const KEY_LEN_MAC   : usize = 32;
const KEY_LEN_CIPHER: usize = 32;
const KEY_LEN_IV    : usize = 16;

const SEED_NULL     : [u8;1] = [0];
const SEED_MSG_KEY  : [u8;1] = [1];
const SEED_CHAIN_KEY: [u8;1] = [2];

pub struct TextSecureV3;

pub struct IdentityKey;

impl axolotl::DH for IdentityKey {
    type Private = curve25519::PrivateKey;
    type Public  = curve25519::PublicKey;
    type Shared  = curve25519::SharedKey;

    fn public(key : &Self::Private) -> Self::Public{
        curve25519::derive_public_key(key)
    }
    fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
        curve25519::derive_shared_key( mine,theirs)
    }
}

pub fn ident_to_ratchet(ident : DHKeyPair<IdentityKey> ) -> DHKeyPair<RatchetKey> {
    DHKeyPair{key: ident.key, public:ident.public}
}

pub struct RatchetKey;
impl axolotl::DH for RatchetKey {
    type Private = curve25519::PrivateKey;
    type Public = curve25519::PublicKey;
    type Shared = curve25519::SharedKey;

    fn public(key : &Self::Private) -> Self::Public{
        curve25519::derive_public_key(key)
    }
    fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared{
        curve25519::derive_shared_key( mine,theirs)
    }

}

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
#[derive(Debug)]
#[derive(Clone)]
pub struct MessageKey{
    cipher_key : [u8;32],
    mac_key : [u8;32],
    iv : [u8;16],
}

#[derive(Debug)]
pub struct PlainText(pub Box<[u8]>);

impl From<Vec<u8>> for PlainText {
    fn from(data: Vec<u8>) -> Self {
        PlainText(data.into_boxed_slice())
    }
}

#[derive(Debug)]
pub struct CipherTextAndVersion{
    cipher_text : Box<[u8]>,
    version : u8,
}

impl From<(u8, Vec<u8>)> for CipherTextAndVersion {
    fn from((version, data): (u8, Vec<u8>)) -> CipherTextAndVersion {
        CipherTextAndVersion {
            cipher_text: data.into_boxed_slice(),
            version: version
        }
    }
}

impl axolotl::Axolotl for TextSecureV3{
    type IdentityKey = IdentityKey;
    type RatchetKey = RatchetKey;

    type RootKey = Rootkey;
    type ChainKey = ChainKey;
    type MessageKey = MessageKey;

    type PlainText = PlainText;
    type CipherText = CipherTextAndVersion;

    type Mac = hmac::MacResult;

    /// Returns initial Root and Chain keys derived from initial the TripleDH handshake.
    fn derive_initial_root_key_and_chain_key(
        local_identity_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>,
        local_handshake_remote_identity_dh_secret : &DHShared<Self::IdentityKey>,
        local_handshake_remote_handshake_dh_secret : &DHShared<Self::IdentityKey>)
            -> (Self::RootKey, Self::ChainKey) {

        let secrets :&[&[u8]] = &[local_identity_remote_handshake_dh_secret.to_bytes(),
                                  local_handshake_remote_identity_dh_secret.to_bytes(),
                                  local_handshake_remote_handshake_dh_secret.to_bytes()];

        let (rk, ck) = keys_from_kdf(&secrets.concat(), b"WhisperText", &SEED_NULL);
        (Rootkey(rk),ChainKey(ck))
    }

    /// Returns new Root and Chain keys derived from racheting previous keyset.
    fn derive_next_root_key_and_chain_key(root_key : Self::RootKey, ratchet : &<Self::RatchetKey as DH>::Shared) -> (Self::RootKey, Self::ChainKey){
        let Rootkey( root_bytes ) = root_key;
        let keys : &[&[u8]] = &[&root_bytes, ratchet.to_bytes()];

        let (rk,ck) = keys_from_kdf(&keys.concat(), b"WhisperRatchet", &root_bytes);

        (Rootkey(rk), ChainKey(ck))
    }

    /// Returns derived Message key for given Chain key as well as the next Chain key to be used.
    fn derive_next_chain_and_message_key(chain_key : &Self::ChainKey) -> (Self::ChainKey, Self::MessageKey){

        let ikm = chain_key.hmac(&SEED_MSG_KEY);
        let msg_key = generate_message_key(&ikm, b"WhisperMessage", &SEED_NULL);
        (chain_key.next(), msg_key)
    }

    fn encrypt_message(message_key : &Self::MessageKey,
                       &PlainText(ref text) : &Self::PlainText)
                       -> Self::CipherText {
        let ciphertext = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);

        (3, ciphertext).into()
    }

    fn decrypt_message(message_key : &Self::MessageKey,
                      ciphertext : &Self::CipherText)
                      -> Option<Self::PlainText>{
        if ciphertext.version != 3 {
            return None;
        }

        let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext.cipher_text,
                                                      message_key.cipher_key,
                                                      message_key.iv);
        Some(result.into())
    }

    fn authenticate_message(message : &AxolotlMessage<Self>,
                            message_key : &Self::MessageKey,
                            sender_identity : &DHPublic<Self::IdentityKey>,
                            receiver_identity : &DHPublic<Self::IdentityKey>)
            -> Self::Mac {
        let mut mac_state = hmac::HmacSha256::new(&message_key.mac_key);
        mac_state.input(sender_identity.to_bytes());
        mac_state.input(receiver_identity.to_bytes());
        mac_state.input(&message.ciphertext.cipher_text[..]); //TODO: input the version
        hmac::truncate_mac_result(mac_state.result(), 8)
    }

    fn generate_ratchet_key_pair() -> DHKeyPair<Self::RatchetKey> {
        let priv_key = curve25519::generate_private_key();
        let pub_key = curve25519::derive_public_key(&priv_key);

        DHKeyPair {
            key: priv_key,
            public : pub_key
        }
    }

    fn future_message_limit() -> u32 {
        2000
    }

    fn chain_message_limit() -> u32 {
        WHOLE_BUNCH
    }

    fn skipped_chain_limit() -> usize {
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
