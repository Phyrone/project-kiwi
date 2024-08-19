use std::ops::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[inline]
pub fn ser_clone<T>(t: &T) -> T
where
    T: Serialize + DeserializeOwned,
{
    bincode::deserialize(&bincode::serialize(t).unwrap()).unwrap()
}