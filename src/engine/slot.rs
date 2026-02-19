use std::marker::PhantomData;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use super::{SerializeError, Serializer, WincodeDecode, WincodeEncode};

pub struct Storage<T, S>
where
    T: BorshSerialize
        + SerdeSerialize
        + BorshDeserialize
        + for<'de> SerdeDeserialize<'de>
        + WincodeEncode
        + WincodeDecode,
    S: Serializer,
{
    serializer: S,
    data: Vec<u8>,
    _marker: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    T: BorshSerialize
        + SerdeSerialize
        + BorshDeserialize
        + for<'de> SerdeDeserialize<'de>
        + WincodeEncode
        + WincodeDecode,
    S: Serializer,
{
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), SerializeError> {
        self.data = self.serializer.to_bytes(value)?;
        Ok(())
    }

    pub fn load(&self) -> Result<T, SerializeError> {
        self.serializer.from_bytes(&self.data)
    }

    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }

    pub fn convert_to_other_format<S2: Serializer>(&self, other_serializer: S2) -> Storage<T, S2> {
        Storage {
            serializer: other_serializer,
            data: self.data.clone(),
            _marker: PhantomData,
        }
    }
}