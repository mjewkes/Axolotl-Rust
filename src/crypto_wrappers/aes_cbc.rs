/// Wrapper for external AES encryption Library
use crypto::{aes, blockmodes, buffer};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::symmetriccipher::Encryptor;

pub const KEY_LEN : usize = 32;
pub const IV_LEN  : usize = 16;

const BUF_SIZE : usize = 4096;

const KEY_PARAM  : aes::KeySize = aes::KeySize::KeySize256;
const BLOCK_MODE : blockmodes::PkcsPadding = blockmodes::PkcsPadding;

pub fn encrypt_aes256_cbc_mode(data: &[u8], key:  [u8; KEY_LEN], iv: [u8;IV_LEN]) -> Vec<u8>{

	let mut encryptor = aes::cbc_encryptor( KEY_PARAM, &key, &iv, BLOCK_MODE);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; BUF_SIZE];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result : BufferResult = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).ok().unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    final_result
}
pub fn decrypt_aes256_cbc_mode(ciphertext: &[u8], key:  [u8; KEY_LEN], iv: [u8;IV_LEN]) -> Vec<u8> {
    let mut decryptor = aes::cbc_decryptor( KEY_PARAM, &key, &iv, BLOCK_MODE);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(ciphertext);
    let mut buffer = [0; BUF_SIZE];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result : BufferResult = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).ok().unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    final_result
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn cbc_encryption() {
        // TODO: MUST EXTEND test to included chained iterations
    	let data = [0xAE ,0x2D ,0x8A ,0x57 ,0x1E ,0x03 ,0xAC ,0x9C ,0x9E ,0xB7 ,0x6F ,0xAC ,0x45 ,0xAF ,0x8E ,0x51 ];
    	let key : [u8; KEY_LEN] = [0x60 ,0x3D ,0xEB ,0x10 ,0x15 ,0xCA ,0x71 ,0xBE ,0x2B ,0x73 ,0xAE ,0xF0 ,0x85 ,0x7D ,0x77 ,0x81 ,0x1F ,0x35 ,0x2C ,0x07 ,0x3B ,0x61 ,0x08 ,0xD7 ,0x2D ,0x98 ,0x10 ,0xA3 ,0x09 ,0x14 ,0xDF ,0xF4];
    	let iv  : [u8; IV_LEN] = [0xF5 ,0x8C ,0x4C ,0x04 ,0xD6 ,0xE5 ,0xF1 ,0xBA ,0x77 ,0x9E ,0xAB ,0xFB ,0x5F ,0x7B ,0xFB ,0xD6 ];
        let expected_ciphertext : [u8;32] = [0x9c ,0xfc ,0x4e,0x96,0x7e,0xdb,0x80,0x8d,0x67,0x9f,0x77,0x7b,0xc6,0x70,0x2c,0x7d,0x3a,0x3a,0xa5,0xe0,0x21,0x3d,0xb1,0xa9,0x90,0x1f,0x90,0x36,0xcf,0x51,0x02,0xd2];

        let ciphertext = encrypt_aes256_cbc_mode(&data,key,iv);

        assert_eq!(expected_ciphertext,&ciphertext[..]);
    }

    #[test]
    fn cbc_roundtrip() {

        let plaintext = [0xAE ,0x2D ,0x8A ,0x57 ,0x1E ,0x03 ,0xAC ,0x9C ,0x9E ,0xB7 ,0x6F ,0xAC ,0x45 ,0xAF ,0x8E ,0x51 ];
        let key : [u8; 32] = [0x60 ,0x3D ,0xEB ,0x10 ,0x15 ,0xCA ,0x71 ,0xBE ,0x2B ,0x73 ,0xAE ,0xF0 ,0x85 ,0x7D ,0x77 ,0x81 ,0x1F ,0x35 ,0x2C ,0x07 ,0x3B ,0x61 ,0x08 ,0xD7 ,0x2D ,0x98 ,0x10 ,0xA3 ,0x09 ,0x14 ,0xDF ,0xF4];
        let iv  : [u8; 16] = [0xF5 ,0x8C ,0x4C ,0x04 ,0xD6 ,0xE5 ,0xF1 ,0xBA ,0x77 ,0x9E ,0xAB ,0xFB ,0x5F ,0x7B ,0xFB ,0xD6 ];

        let cipher_text = encrypt_aes256_cbc_mode(&plaintext,key,iv);

        let derived_plaintext = decrypt_aes256_cbc_mode(&cipher_text[..],key,iv);

        assert_eq!(&derived_plaintext[..],plaintext);
    }
    #[test]
    fn cbc_improper_key(){
        //TODO: IMPLEMENT TEST
    }



}
