use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

#[inline]
pub fn ser_clone<T>(t: &T) -> T
where
    T: Serialize + DeserializeOwned,
{
    bincode::deserialize(&bincode::serialize(t).unwrap()).unwrap()
}
