pub use protobuf::core::Message;

    use whisper_protocol::crypto_wrappers::{aes_cbc,curve25519,hkdf,hmac};
pub use whisper_protocol::raxolotl::axolotl::{Axolotl,Header,KeyPair};
pub use super::WhisperTextProtocol::WhisperMessage;


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

const TRUNCATED_MAC_LEN : usize = 8;

const KEY_LEN_CHAIN : usize = 32;
const KEY_LEN_ROOT  : usize = 32;
const KEY_LEN_MAC   : usize = 32;
const KEY_LEN_CIPHER: usize = 32;
const KEY_LEN_IV    : usize = 16;

const SEED_NULL     : [u8;1] = [0]; 
const SEED_MSG_KEY  : [u8;1] = [1];
const SEED_CHAIN_KEY: [u8;1] = [2];
#[derive(RustcEncodable, RustcDecodable)]
pub struct TextSecureV3;
impl TextSecureV3 {
    #[inline]
    pub fn version_num(&self) -> u8 { 3 }
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Rootkey ([u8;32]);

#[derive(Clone, RustcEncodable, RustcDecodable)]
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
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct MessageKey{
    cipher_key : [u8;32],
    mac_key : [u8;32],
    iv : [u8;16],
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct TransportPacket {
    version: u8,
    payload : WhisperMessage,
    mac : Option<Vec<u8>>,
}

impl TransportPacket {
    pub fn new(version : u8 ) -> Self {
        TransportPacket{
            version:version,
            payload : WhisperMessage::new(),
            mac : None,
        }
    }

    pub fn from_bytes( bytes : &[u8]) -> Result<Self,()> {
        let (version, payload_bytes, mac) = TransportPacket::partition_packed_bytes(bytes);

        let mut tp = TransportPacket::new(deserialize_version(version));
        if tp.payload.merge_from_bytes(payload_bytes).is_err() {
            return Err(());
        }
        tp.mac = Some(mac.to_vec());
        
        Ok(tp)
    }

    pub fn set_mac(self : &mut Self,mac : Vec<u8>){
        self.mac = Some(mac);
    }

    pub fn take_mac(self : &mut Self) -> Option<Vec<u8>>{
        self.mac.take()
    }
    
    pub fn to_vec(self : Self) -> Vec<u8>{
        let mut v = Vec::<u8>::new();
        v.push(serialize_version(self.version));

        let payload = self.payload.write_to_bytes().ok().unwrap();
        for x in payload {
            v.push(x);
        }
        for x in self.mac.unwrap(){
            v.push(x);
        }
        v
    }

    fn partition_packed_bytes(bytes: &[u8]) -> (u8,&[u8],&[u8]) {
        (bytes[0], &bytes[1..bytes.len() - TRUNCATED_MAC_LEN], &bytes[bytes.len() - TRUNCATED_MAC_LEN..] )
    }
}

#[derive(Clone)]
pub struct PlainText(pub Vec<u8>);

impl PlainText {
    pub fn from_vec(data : Vec<u8>) -> PlainText {
        PlainText(data)
    }
}

impl Axolotl for TextSecureV3{
    type PrivateKey = curve25519::PrivateKey;
    type PublicKey  = curve25519::PublicKey;
    type SharedSecret  = curve25519::SharedKey;

    type InitialSharedSecret = Vec<u8>;
    type SessionIdentity = Vec<u8>;

    type RootKey = Rootkey;
    type ChainKey = ChainKey;
    type MessageKey = MessageKey;

    type PlainText = PlainText;
    type CipherText = Vec<u8>;
    type Message = TransportPacket;

    type Mac = Vec<u8>;

    type EncryptError = ();
    type EncodeError = ();
    type DecryptError = ();
    type DecodeError = ();

    /// Returns initial Root and Chain keys derived from initial the TripleDH handshake. 
    fn derive_initial_root_key_and_chain_key(
        &self,
        initial_secret : Self::InitialSharedSecret
    ) -> (Self::RootKey, Self::ChainKey){

        let master_key = [&[0xFF;32][..], &initial_secret[..]].concat();
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
        plaintext : Self::PlainText
    ) -> Result<Self::CipherText, Self::EncryptError> {

        let PlainText(ref text) = plaintext;
        let ciphertext = aes_cbc::encrypt_aes256_cbc_mode(text,message_key.cipher_key, message_key.iv);
        
        Ok(ciphertext)
    }

    fn decrypt_message(
        &self,
        message_key : &Self::MessageKey, 
        ciphertext : Self::CipherText
    ) -> Result<Self::PlainText,()>{
        let result = aes_cbc::decrypt_aes256_cbc_mode(&ciphertext, message_key.cipher_key, message_key.iv);
        Ok(PlainText(result))
    }

    fn authenticate_message(
        &self,
        message : &Self::Message, 
        message_key : &Self::MessageKey, 
        session_identity : &Self::SessionIdentity
    ) -> Self::Mac{

        let mut serialized = Vec::<u8>::new();
        serialized.push(serialize_version(self.version_num()));
        message.payload.write_to_vec(&mut serialized).ok();
       
        let mut mac_state = hmac::HmacSha256::new(&message_key.mac_key);
        mac_state.input(&session_identity[..]);
        mac_state.input(&serialized[..]);
        hmac::truncate_mac_result(mac_state.result(), 8).code().to_vec()
    }

    fn encode_header_and_ciphertext(
        &self,
        header : Header<Self>,
        ciphertext : Self::CipherText
    ) -> Result<Self::Message, Self::EncodeError> {
        let mut msg = TransportPacket::new(self.version_num());

        msg.payload.set_counter(header.message_number as u32);
        msg.payload.set_previousCounter(header.message_number_prev as u32);
        msg.payload.set_ciphertext(ciphertext);

        let ratchet = header.ratchet_key.to_bytes();
        let mut v = Vec::<u8>::with_capacity(ratchet.len());
        for x in ratchet.iter(){
            v.push(*x);
        }
        msg.payload.set_ratchetKey(v);         // Can we move from the Header

        Ok(msg)
    }

    fn decode_header(&self, message : &Self::Message
    ) -> Result<Header<Self>,Self::DecodeError> {
        Ok(Header{ 
            message_number : message.payload.get_counter() as usize, 
            message_number_prev : message.payload.get_previousCounter() as usize, 
            ratchet_key : Self::PublicKey::copy_from_bytes(message.payload.get_ratchetKey()),
        })
    }

    fn decode_ciphertext(&self, message : Self::Message
    ) -> Result<Self::CipherText,()> {
        let mut m = message;
        Ok(m.payload.take_ciphertext())
    }

    fn ratchet_keys_are_equal(&self, key0 : &Self::PublicKey, key1 : &Self::PublicKey) -> bool{
        key0 == key1
    }
    fn generate_ratchet_key_pair(&self) -> KeyPair<Self>{
        let priv_key  = curve25519::generate_private_key();
        let pub_key = curve25519::derive_public_key(&priv_key);

        KeyPair{ key: priv_key, public : pub_key }
    }

    fn derive_shared_secret(&self, mine : &Self::PrivateKey, theirs : &Self::PublicKey) -> Self::SharedSecret {
        curve25519::derive_shared_key( mine,theirs)
    }

    fn future_message_limit(&self) -> usize{
        2000
    }
    fn chain_message_limit(&self) -> usize{
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

pub fn serialize_version(version : u8) -> u8{
    (version << 4 | 3) as u8 
}
pub fn deserialize_version(serialized : u8) -> u8{
    (serialized >> 4 ) as u8 
}
