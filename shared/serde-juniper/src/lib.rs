use base64::alphabet::STANDARD;
use base64::Engine;
use base64::engine::general_purpose::PAD;
use base64::engine::GeneralPurpose;
use juniper::{DefaultScalarValue, GraphQLScalar, Object, ScalarValue, Value};
use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;
use thiserror::Error;

#[derive(GraphQLScalar)]
#[graphql(with = json_scalar)]
pub struct JSON<T>(T);

mod json_scalar {
    use juniper::{InputValue, ParseScalarResult, ScalarToken, ScalarValue, Value};

    use crate::JSON;

    pub(super) fn to_output<S: ScalarValue, T>(v: &JSON<T>) -> Value<S> {
        todo!()
    }

    pub(super) fn from_input<S: ScalarValue, T>(v: &InputValue<S>) -> Result<JSON<T>, String> {
        todo!()
    }

    pub(super) fn parse_token<S: ScalarValue>(t: ScalarToken<'_>) -> ParseScalarResult<S> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("JuniperSerializerError")]
pub struct JuniperSerializerError;

impl serde::ser::Error for JuniperSerializerError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        todo!()
    }
}


static B64_ENGINE: GeneralPurpose = GeneralPurpose::new(&STANDARD, PAD);

pub struct JuniperSerializer;

impl Serializer for JuniperSerializer {
    type Ok = Value;
    type Error = JuniperSerializerError;
    type SerializeSeq = JuniperSeqSerializer;
    type SerializeTuple = ();
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();
    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Boolean(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Int(v.into())))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Int(v.into())))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Int(v.into())))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if v > i32::MAX as i64 || v < i32::MIN as i64 {
            Ok(Value::Scalar(DefaultScalarValue::String(v.to_string())))
        } else {
            Ok(Value::Scalar(DefaultScalarValue::Int(v as i32)))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Int(v.into())))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Int(v.into())))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if v > i32::MAX as u32 {
            Ok(Value::Scalar(DefaultScalarValue::String(v.to_string())))
        } else {
            Ok(Value::Scalar(DefaultScalarValue::Int(v as i32)))
        }
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v > i32::MAX as u64 {
            Ok(Value::Scalar(DefaultScalarValue::String(v.to_string())))
        } else {
            Ok(Value::Scalar(DefaultScalarValue::Int(v as i32)))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Float(v as f64)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::Float(v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::String(v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::String(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Scalar(DefaultScalarValue::String(B64_ENGINE.encode(v))))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Object(Object::with_capacity(0)))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        let mut outer = Object::with_capacity(1);
        let inner = Object::with_capacity(0);
        outer.add_field(name, Value::Object(inner));
        Ok(Value::Object(outer))
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub struct JuniperSeqSerializer {
    items: Vec<Value>,
}
impl JuniperSeqSerializer {
    pub fn new(
        capacity: Option<usize>,
    ) -> Self {
        Self {
            items: if let Some(capacity) = capacity {
                Vec::with_capacity(capacity)
            } else {
                Vec::new()
            },
        }
    }
}

impl SerializeSeq for JuniperSeqSerializer {
    type Ok = Value;
    type Error = JuniperSerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.items.push(value.serialize(JuniperSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::List(self.items))
    }
}