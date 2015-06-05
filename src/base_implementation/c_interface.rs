
use super::base_implementation::{BaseImplementation, KEY_LEN, KEY_LEN_IV};
use std::boxed;

pub static ERROR_OK : i32 = 0;

pub type KeyGenFunc  = fn(&u8, &u8, usize ) -> i32;
pub type PubKeyFunc  = fn(&u8, &u8, usize ) -> i32; 
pub type DhFunc      = fn(&u8, &u8, &u8, usize)  -> i32;
pub type EncryptFunc = fn( &u8, usize, &u8, &u8, usize , *mut u8, &usize) -> i32;
pub type DecryptFunc = fn( &u8, usize, &u8, &u8, usize , *mut u8, &usize) -> i32;

#[no_mangle]
pub unsafe extern fn create_axolotl_context(
    genkey : KeyGenFunc,
    pubkey : PubKeyFunc,
    dhke : DhFunc,
    enc : EncryptFunc,
    dec : DecryptFunc,
    kdf_info_init : *mut u8,  kdf_info_init_len : u32,
    kdf_info_ratchet : *mut u8, kdf_info_ratchet_len : u32,
    kdf_info_msg : *mut u8, kdf_info_msg_len : u32, 
) -> *mut AxolotlContext{
    let init_string : Vec<u8> = Vec::<u8>::from_raw_parts(kdf_info_init,kdf_info_init_len as usize,kdf_info_init_len as usize);
    let ratchet_string : Vec<u8> = Vec::<u8>::from_raw_parts(kdf_info_ratchet,kdf_info_init_len as usize,kdf_info_ratchet_len as usize);
    let msg_string : Vec<u8> = Vec::<u8>::from_raw_parts(kdf_info_msg,kdf_info_msg_len as usize,kdf_info_msg_len as usize);

    let obj = AxolotlContext{
        fn_gen_key_pair : genkey,
        fn_pub_key : pubkey,
        fn_dhke : dhke,
        fn_enc : enc,
        fn_dec : dec,
        kdf_info_init : init_string,
        kdf_info_ratchet : ratchet_string,
        kdf_info_msg : msg_string,
    };

    let b = Box::<AxolotlContext>::new(obj);
    boxed::into_raw(b)
}

#[no_mangle]
pub unsafe extern fn destroy_axolotl_context(axo : *mut AxolotlContext){
    Box::from_raw(axo);
}

#[repr(C)]
pub struct AxolotlContext {
    pub fn_gen_key_pair :   KeyGenFunc,
    pub fn_pub_key : PubKeyFunc,
    pub fn_dhke : DhFunc,
    pub fn_enc : EncryptFunc,
    pub fn_dec : DecryptFunc,
    pub kdf_info_init :     Vec<u8>,
    pub kdf_info_ratchet :  Vec<u8>,
    pub kdf_info_msg :      Vec<u8>,
}

impl BaseImplementation for AxolotlContext {
    fn gen_key_pair(self: &Self) ->  ([u8;KEY_LEN],[u8;KEY_LEN]){

        let private : [u8;32] = [0;KEY_LEN];
        let public : [u8;32] = [0;KEY_LEN];

        (self.fn_gen_key_pair)(&private[0],&public[0], KEY_LEN);
        (private,public)
    }

    fn gen_pub_key(self: &Self,private : [u8; KEY_LEN] ) -> ( [u8;KEY_LEN]){
        let public : [u8;32] = [0;KEY_LEN];
        (self.fn_pub_key)(&private[0], &public[0],KEY_LEN);
        public
    }

    fn dh_key_exchange(self: &Self, local_private :  [u8; KEY_LEN], remote_pub: [u8; KEY_LEN] ) -> ([u8;KEY_LEN]){
        let shared : [u8;32] = [0;KEY_LEN]; 
        
        let error : i32 = (self.fn_dhke)(&local_private[0],&remote_pub[0], &shared[0], KEY_LEN);
        assert_eq!(error,ERROR_OK);
        
        shared
    }

    fn enc_bytes(self: &Self, data : &[u8], key : [u8;KEY_LEN], iv : [ u8; KEY_LEN_IV] ) -> (Vec<u8>){
        let byte_len : usize = 0;
        let mut encrypted_bytes = Vec::<u8>::with_capacity(encrypted_msg_length(data.len()));
        unsafe {
            (self.fn_enc)(&data[0], data.len(),&key[0],&iv[0], KEY_LEN, encrypted_bytes.as_ptr() as *mut u8, &byte_len);
            encrypted_bytes.set_len(byte_len);
        }
        encrypted_bytes
    }

    fn dec_bytes( self: &Self,ciphertext : &[u8], key : [u8;KEY_LEN], iv : [ u8; KEY_LEN_IV] ) -> (Vec<u8>){
        let byte_len : usize = 0;
        let mut decrypted_bytes = Vec::<u8>::with_capacity(ciphertext.len());
        unsafe {
            (self.fn_dec)(&ciphertext[0], ciphertext.len(),&key[0],&iv[0], KEY_LEN, decrypted_bytes.as_ptr() as *mut u8, &byte_len);
            decrypted_bytes.set_len(byte_len);
        }
        decrypted_bytes
    }

    fn get_masterkey_prefix(self: &Self) -> Option<[u8;KEY_LEN]>{
            Some([0xFF;KEY_LEN])
    }

    fn kdf_info_init(self: &Self) ->  &[u8] {
        &self.kdf_info_init[..]
    }

    fn kdf_info_ratchet(self: &Self) ->  &[u8]{
        &self.kdf_info_ratchet[..]
    }

    fn kdf_info_msg(self: &Self) ->  &[u8]{
        &self.kdf_info_msg[..]
    }
}

const AES_BLOCK_SIZE : usize = 128;
fn encrypted_msg_length(plaintext_len : usize ) -> usize{
    (plaintext_len-1) / AES_BLOCK_SIZE + 1
}

#[cfg(test)]
mod tests {
    
}