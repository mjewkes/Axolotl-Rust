extern crate crypto;
use crypto::{  aes, blockmodes ,buffer,symmetriccipher};
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub const KEY_LEN : usize = 32;
pub const IV_LEN  : usize = 16;

const BUF_SIZE : usize = 4096;

// TODO: Handle Error Propagation/ Wrapping
pub fn encrypt_aes256_cbc_mode(data: &[u8], key:  [u8; KEY_LEN], iv: [u8;IV_LEN]) -> Result<Vec<u8>,  symmetriccipher::SymmetricCipherError>{

	let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            &key,
            &iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; BUF_SIZE];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result : BufferResult = match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true){
            Err(why) => panic!(),
            Ok(val)  => val
        };
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
        
    }

    Ok(final_result)
}
// TODO: Handle Error Propagation/ Wrapping
pub fn decrypt_aes256_cbc_mode(ciphertext: &[u8], key:  [u8; KEY_LEN], iv: [u8;IV_LEN]) -> Result<Vec<u8>,  symmetriccipher::SymmetricCipherError>{
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            &key,
            &iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(ciphertext);
    let mut buffer = [0; BUF_SIZE];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {

        let result : BufferResult = match decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            Err(why) => panic!(),
            Ok(val)  => val
        };
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
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
        
        let ciphertext = match encrypt_aes256_cbc_mode(&data,key,iv){
            Err(why) => panic!(),
            Ok(v) =>  v
        };

        assert_eq!(expected_ciphertext,&ciphertext[..]);

    }

    #[test]
    fn cbc_roundtrip() {

        let plaintext = [0xAE ,0x2D ,0x8A ,0x57 ,0x1E ,0x03 ,0xAC ,0x9C ,0x9E ,0xB7 ,0x6F ,0xAC ,0x45 ,0xAF ,0x8E ,0x51 ];
        let key : [u8; 32] = [0x60 ,0x3D ,0xEB ,0x10 ,0x15 ,0xCA ,0x71 ,0xBE ,0x2B ,0x73 ,0xAE ,0xF0 ,0x85 ,0x7D ,0x77 ,0x81 ,0x1F ,0x35 ,0x2C ,0x07 ,0x3B ,0x61 ,0x08 ,0xD7 ,0x2D ,0x98 ,0x10 ,0xA3 ,0x09 ,0x14 ,0xDF ,0xF4];
        let iv  : [u8; 16] = [0xF5 ,0x8C ,0x4C ,0x04 ,0xD6 ,0xE5 ,0xF1 ,0xBA ,0x77 ,0x9E ,0xAB ,0xFB ,0x5F ,0x7B ,0xFB ,0xD6 ];
       
        let cipher_text = match encrypt_aes256_cbc_mode(&plaintext,key,iv){
            Err(why) => panic!(),
            Ok(v) =>  v
        };

        let derived_plaintext = match decrypt_aes256_cbc_mode(&cipher_text[..],key,iv) {
            Err(why) => {println!("{:?}",why ); panic!()},
            Ok(v) => v
        };

        assert_eq!(&derived_plaintext[..],plaintext);

    }
    #[test]
    fn cbc_improper_key(){
        //TODO: IMPLEMENT TEST
    }



}