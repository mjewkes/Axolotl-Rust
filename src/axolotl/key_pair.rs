use super::axolotl::Axolotl;

pub struct KeyPair<T> where T:Axolotl {
    pub key : T::PrivateKey,
    pub public : T::PublicKey,
}

impl_axolotl_serde!{
	key_pair_serde { 
		KeyPair { key, public }
	}
}