use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct CloneWrapper<T>(T) where T: Serialize + DeserializeOwned;

impl<T> Clone for CloneWrapper<T>
where
    T: Serialize + DeserializeOwned,
{
    fn clone(&self) -> Self {
        
        todo!()
    }
}