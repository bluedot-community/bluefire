// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of 12 byte ID data structure.

// TODO: Add tests.

use byteorder::ByteOrder;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const ID_SIZE_IN_BYTES: usize = 12;

/// Enumeration describing conversion errors.
#[derive(Debug)]
pub enum IdError {
    /// Converting using `hex` crate failed.
    FromHexError(hex::FromHexError),

    /// The passed string has wrong length.
    WrongLength {
        /// The length of the passed string.
        is: usize,
        /// The length expected from a string containing a valid ID.
        expected: usize,
    },
}

impl From<hex::FromHexError> for IdError {
    fn from(err: hex::FromHexError) -> IdError {
        IdError::FromHexError(err)
    }
}

impl std::fmt::Display for IdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IdError::FromHexError(err) => write!(f, "{}", err),
            IdError::WrongLength { is, expected } => {
                write!(f, "The length '{}' is wrong. Expected '{}'.", is, expected)
            }
        }
    }
}

/// A container for 12 byte IDs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    data: [u8; ID_SIZE_IN_BYTES],
}

impl Id {
    /// Constructs a new random `Id`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_random() -> Self {
        use rand::RngCore;
        let mut data: [u8; ID_SIZE_IN_BYTES] = [0; ID_SIZE_IN_BYTES];
        let timestamp = chrono::Utc::now().timestamp() as u32;
        byteorder::BigEndian::write_u32(&mut data[0..4], timestamp);
        rand::thread_rng().fill_bytes(&mut data[0..ID_SIZE_IN_BYTES]);
        Id { data }
    }

    /// Constructs a new random `Id`.
    #[cfg(target_arch = "wasm32")]
    pub fn new_random() -> Self {
        let mut data: [u8; ID_SIZE_IN_BYTES] = [0; ID_SIZE_IN_BYTES];

        let timestamp = js_sys::Date::now() as u32;
        byteorder::BigEndian::write_u32(&mut data[0..4], timestamp);

        let rand = (std::u64::MAX as f64 * js_sys::Math::random()) as u64;
        byteorder::BigEndian::write_u64(&mut data[4..ID_SIZE_IN_BYTES], rand);

        Id { data }
    }

    /// Constructs a new `Id` from a string. The given string has to be 12 bytes long.
    pub fn from_str(id: &str) -> Result<Self, IdError> {
        let bytes: Vec<u8> = hex::decode(id.as_bytes())?;
        if bytes.len() == ID_SIZE_IN_BYTES {
            let mut data: [u8; ID_SIZE_IN_BYTES] = [0; ID_SIZE_IN_BYTES];
            for i in 0..ID_SIZE_IN_BYTES {
                data[i] = bytes[i];
            }
            Ok(Id { data })
        } else {
            Err(IdError::WrongLength { is: bytes.len(), expected: ID_SIZE_IN_BYTES })
        }
    }

    /// Constructs a new `Id` from a string. The given string has to be 12 bytes long.
    ///
    /// Panics if fails it's not possible to convert.
    pub fn from_str_unchecked(id: &str) -> Self {
        Self::from_str(id).expect("Failed to convert string into Id")
    }

    /// Returns a string representation.
    pub fn to_hex(&self) -> String {
        hex::encode(self.data)
    }

    /// Casts the `Id` into `bson::oid::ObjectId`.
    #[cfg(feature = "bson_conversion")]
    pub fn into_bson_oid(&self) -> bson::oid::ObjectId {
        bson::oid::ObjectId::with_string(&self.to_hex())
            .expect("Cast from bluefire Id to bson ObjectId")
    }

    /// Casts the `bson::oid::ObjectId` into `Id`.
    #[cfg(feature = "bson_conversion")]
    pub fn from_bson_oid(oid: &bson::oid::ObjectId) -> Self {
        Id::from_str(&oid.to_hex()).expect("Cast from bson ObjectId to bluefire Id")
    }

    /// Casts the `Id` into `bson::Bson`.
    #[cfg(feature = "bson_conversion")]
    pub fn into_bson(&self) -> bson::Bson {
        bson::Bson::ObjectId(self.into_bson_oid())
    }

    /// Casts the `bson::Bson` into `Id`.
    #[cfg(feature = "bson_conversion")]
    pub fn from_bson(bson: &bson::Bson) -> Self {
        match bson {
            bson::Bson::ObjectId(oid) => Id::from_bson_oid(oid),
            bson::Bson::String(string) => {
                Id::from_str(string).expect("Cast from bson to bluefire Id")
            }
            _ => panic!("Cast from wrong variant of bson to bluefire Id"),
        }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Id, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Id::from_str(&s).map_err(D::Error::custom)
    }
}

impl std::cmp::PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.to_hex() == *other
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(feature = "bson_conversion")]
impl From<&Id> for bson::oid::ObjectId {
    fn from(id: &Id) -> bson::oid::ObjectId {
        id.into_bson_oid()
    }
}

#[cfg(feature = "bson_conversion")]
impl From<&bson::oid::ObjectId> for Id {
    fn from(oid: &bson::oid::ObjectId) -> Id {
        Id::from_bson_oid(oid)
    }
}

#[cfg(feature = "bson_conversion")]
impl From<Id> for bson::oid::ObjectId {
    fn from(id: Id) -> bson::oid::ObjectId {
        id.into_bson_oid()
    }
}

#[cfg(feature = "bson_conversion")]
impl From<bson::oid::ObjectId> for Id {
    fn from(oid: bson::oid::ObjectId) -> Id {
        Id::from_bson_oid(&oid)
    }
}
