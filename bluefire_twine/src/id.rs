// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of 12 byte ID data structure.

// TODO: Add tests.

use rand::RngCore;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const ID_SIZE_IN__BYTES: usize = 12;

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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    data: [u8; ID_SIZE_IN__BYTES],
}

impl Id {
    /// Constructs a new random `Id`.
    pub fn new_random() -> Self {
        let mut data: [u8; ID_SIZE_IN__BYTES] = [0; ID_SIZE_IN__BYTES];
        rand::thread_rng().fill_bytes(&mut data);
        Id { data }
    }

    /// Constructs a new `Id` from a string. The given string has to be 12 bytes long.
    pub fn from_str(id: &str) -> Result<Self, IdError> {
        let bytes: Vec<u8> = hex::decode(id.as_bytes())?;
        if bytes.len() == ID_SIZE_IN__BYTES {
            let mut data: [u8; ID_SIZE_IN__BYTES] = [0; ID_SIZE_IN__BYTES];
            for i in 0..ID_SIZE_IN__BYTES {
                data[i] = bytes[i];
            }
            Ok(Id { data })
        } else {
            Err(IdError::WrongLength { is: bytes.len(), expected: ID_SIZE_IN__BYTES })
        }
    }

    /// Returns a string representation.
    pub fn to_hex(&self) -> String {
        hex::encode(self.data)
    }

    /// Casts the `Id` into `bson::oid::ObjectId`.
    #[cfg(feature = "bson_conversion")]
    pub fn into_bson_oid(self) -> bson::oid::ObjectId {
        bson::oid::ObjectId::with_string(&self.to_hex())
            .expect("Cast from bluefire Id to bson ObjectId")
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

#[cfg(feature = "bson_conversion")]
impl From<Id> for bson::oid::ObjectId {
    fn from(id: Id) -> bson::oid::ObjectId {
        id.into_bson_oid()
    }
}

#[cfg(feature = "bson_conversion")]
impl From<bson::oid::ObjectId> for Id {
    fn from(oid: bson::oid::ObjectId) -> Id {
        Id::from_str(&oid.to_hex()).expect("Cast from bson ObjectId to bluefire Id")
    }
}
