use rustc_serialize::{Encodable,Encoder,Decodable,Decoder};
use super::axolotl::Axolotl;

pub struct KeyPair<T> where T:Axolotl {
    pub key : T::PrivateKey,
    pub public : T::PublicKey,
}

impl<T:Axolotl> Encodable for KeyPair<T> {
	fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
    	s.emit_struct("KeyPair", 2, |s| {
    		try!(s.emit_struct_field("key", 0, |s| {
    			self.key.encode(s)
    		}));
    		try!(s.emit_struct_field("public", 1, |s| {
    			self.public.encode(s)
    		}));
    		Ok(())
    	})
    }
}

impl<T:Axolotl> Decodable for KeyPair<T> {
	fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
		d.read_struct("KeyPair", 2, |d| {
			let key = try!(d.read_struct_field("key", 0, |d| {
				Decodable::decode(d)
			}));
			let public = try!(d.read_struct_field("public", 1, |d| {
				Decodable::decode(d)
			}));
			Ok(KeyPair{key:key,public:public})
		})
	}
}