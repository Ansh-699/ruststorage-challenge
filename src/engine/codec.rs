use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use wincode::config::DefaultConfig;

#[derive(Debug)]
pub struct SerializeError {
    pub message: String,
}

impl SerializeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub trait WincodeDecode: for<'de> wincode::SchemaRead<'de, DefaultConfig, Dst = Self> {}
impl<T> WincodeDecode for T where T: for<'de> wincode::SchemaRead<'de, DefaultConfig, Dst = Self> {}

pub trait WincodeEncode: wincode::SchemaWrite<DefaultConfig, Src = Self> {}
impl<T> WincodeEncode for T where T: wincode::SchemaWrite<DefaultConfig, Src = Self> {}

pub trait Serializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, SerializeError>
    where
        T: BorshSerialize + SerdeSerialize + WincodeEncode;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, SerializeError>
    where
        T: BorshDeserialize + for<'de> SerdeDeserialize<'de> + WincodeDecode;
}

pub struct Borsh;

impl Serializer for Borsh {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, SerializeError>
    where
        T: BorshSerialize + SerdeSerialize + WincodeEncode,
    {
        borsh::to_vec(value).map_err(|err| SerializeError::new(&err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, SerializeError>
    where
        T: BorshDeserialize + for<'de> SerdeDeserialize<'de> + WincodeDecode,
    {
        T::try_from_slice(bytes).map_err(|err| SerializeError::new(&err.to_string()))
    }
}

pub struct Wincode;

impl Serializer for Wincode {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, SerializeError>
    where
        T: BorshSerialize + SerdeSerialize + WincodeEncode,
    {
        wincode::serialize(value).map_err(|err| SerializeError::new(&err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, SerializeError>
    where
        T: BorshDeserialize + for<'de> SerdeDeserialize<'de> + WincodeDecode,
    {
        wincode::deserialize(bytes).map_err(|err| SerializeError::new(&err.to_string()))
    }
}

pub struct Json;

impl Serializer for Json {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, SerializeError>
    where
        T: BorshSerialize + SerdeSerialize + WincodeEncode,
    {
        serde_json::to_vec(value).map_err(|err| SerializeError::new(&err.to_string()))
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, SerializeError>
    where
        T: BorshDeserialize + for<'de> SerdeDeserialize<'de> + WincodeDecode,
    {
        serde_json::from_slice(bytes).map_err(|err| SerializeError::new(&err.to_string()))
    }
}