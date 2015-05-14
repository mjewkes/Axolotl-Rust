pub trait DH {
    type Private : Clone;
    type Public : Clone;
    type Shared;

    fn public(key : &Self::Private) -> Self::Public;
    fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared;
}

//these types are to avoid "<T::SomeKey as DH>::Private", replacing it with "DHPrivate<T::SomeKey>"
pub type DHPrivate<T> where T:DH = T::Private;
pub type DHPublic<T> where T:DH = T::Public;
pub type DHShared<T> where T:DH = T::Shared;

pub struct DHKeyPair<T> where T:DH {
    pub key : T::Private,
    pub public : T::Public,
}

impl <T:DH> Clone for DHKeyPair<T> {
    fn clone(&self) -> Self {
        DHKeyPair {
            key : self.key.clone(),
            public : self.public.clone(),
        }
    }
}

pub struct DHExchangedPair<T> where T:DH {
    pub mine : T::Private,
    pub theirs : T::Public,
}