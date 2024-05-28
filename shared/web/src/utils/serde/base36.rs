use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use num::Num;
use radix_fmt::Radix;
use serde::{de, ser, Serialize};
use serde::de::Error;

const BASE36: u8 = 36;

pub fn serialize<S, V>(dt: &V, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        Radix<V>: Display,
        V: num::Num + num::PrimInt+Serialize,
{
    let b36 = radix_fmt::radix(dt.clone(), BASE36).to_string();
    serializer.serialize_str(&b36)
}

pub fn deserialize<'de, D, V>(d: D) -> Result<V, D::Error>
    where
        D: de::Deserializer<'de>,
        V: Num<FromStrRadixErr=ParseIntError>,
{
    d.deserialize_str(Base36Visitor {
        _phantom: std::marker::PhantomData,
    })
}

struct Base36Visitor<V> where V: Num {
    _phantom: std::marker::PhantomData<V>,
}

impl<'de, V> de::Visitor<'de> for Base36Visitor<V>
    where
        V: Num<FromStrRadixErr=ParseIntError>,

{
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a base36 string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
        V::from_str_radix("0", BASE36 as u32)
            .map_err(E::custom)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        <V as Num>::from_str_radix(v, BASE36 as u32)
            .map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
    {
        <V as Num>::from_str_radix(&v, BASE36 as u32)
            .map_err(E::custom)
    }
}
