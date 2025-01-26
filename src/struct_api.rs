//! Provides the `DecSixbit` struct for encapsulated handling of SIXBIT-encoded data.
//!
//! The `DecSixbit` struct offers a more feature-rich and structured API for encoding and decoding operations,
//! leveraging the underlying encoding and decoding functions.
//!
//! ## Features
//! - Encapsulates SIXBIT-encoded data and its metadata.
//! - Implements common traits for ease of use.
//! - Provides both encoding and decoding functionalities.

use crate::{encode::encode, decode::decode_unchecked, Error};
use std::fmt;

/// The `DecSixbit` struct stores the encoded bytes and provides methods
/// for accessing the encoded data and retrieving the original string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DecSixbit {
    /// Original string length
    pub(crate) len: usize,
    /// Packed bytes where every 3 bytes contain 4 characters (6 bits each)
    pub(crate) bytes: Vec<u8>,
}

impl DecSixbit {
    /// The marker byte for trailing spaces in the last block is added when the length is a multiple of 4, and the last 6 bits are all zero.
    const TRAILING_SPACE_MARKER: u8 = 0b11;

    /// Creates a new DecSixbit instance by encoding the input string.
    /// Only accepts ASCII characters in the range 32-95 (space through underscore).
    /// Creates a new `DecSixbit` instance by encoding the input string.
    ///
    /// # Parameters
    /// - `str`: The input string to encode. Must contain only ASCII characters in the range 32-95.
    ///
    /// # Errors
    /// Returns an [`Error::InvalidCharacter`] if the input contains invalid characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// ```
    pub fn new(str: &str) -> Result<Self, Error> {
        let (mut bytes, len) = encode(str)?;
        // Check if TRAILING_SPACE_MARKER needs to be added
        if len % 4 == 0 && len != 0 && (bytes.last().unwrap() & 0b111111) == 0 {
            bytes.push(Self::TRAILING_SPACE_MARKER);
        }
        Ok(Self { bytes, len })
    }

    /// Returns a reference to the encoded SIXBIT bytes.
    ///
    /// # Returns
    /// A slice of bytes containing the SIXBIT-encoded data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// let encoded = sixbit.as_bytes();
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the length of the original input string.
    ///
    /// # Returns
    /// The number of characters in the original string before encoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// assert_eq!(sixbit.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the encoded SIXBIT data is empty.
    ///
    /// # Returns
    /// `true` if the original input string was empty, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("").unwrap();
    /// assert!(sixbit.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Attempts to create a `DecSixbit` instance from a slice of encoded bytes.
    ///
    /// # Parameters
    /// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
    ///
    /// # Returns
    /// - `Ok(Self)` if the slice is successfully parsed.
    /// - `Err(Error)` if the slice has an invalid format or contains invalid data.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let num_full_blocks = bytes.len() / 3;
        let num_remain_bytes = bytes.len() % 3;

        let len = match num_remain_bytes {
            0 => num_full_blocks * 4,
            1 => {
                if bytes.last().unwrap() == &Self::TRAILING_SPACE_MARKER {
                    num_full_blocks * 4
                } else {
                    num_full_blocks * 4 + 1
                }
            },
            2 => num_full_blocks * 4 + 2,
            _ => unreachable!(),
        };
        Ok(Self {
            len,
            bytes: bytes.to_vec(),
        })
    }

    /// Creates a `DecSixbit` instance from a slice of encoded bytes.
    ///
    /// # Parameters
    /// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
    ///
    /// # Panics
    /// - Panics if the slice has an invalid format or contains invalid data.
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::try_from_slice(bytes).unwrap()
    }

    /// Gets the character at the specified position.
    ///
    /// # Parameters
    /// - `index`: The position of the character to retrieve.
    ///
    /// # Returns
    /// An `Option<char>` which is `Some(char)` if the index is valid, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// assert_eq!(sixbit.get(1), Some('E'));
    /// assert_eq!(sixbit.get(5), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<char> {
        self.to_string().chars().nth(index)
    }

    /// Checks if the string starts with the given prefix.
    ///
    /// # Parameters
    /// - `prefix`: The prefix string to check.
    ///
    /// # Returns
    /// `true` if the string starts with the given prefix, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// assert!(sixbit.starts_with("HE"));
    /// assert!(!sixbit.starts_with("EL"));
    /// ```
    pub fn starts_with<P: AsRef<str>>(&self, prefix: P) -> bool {
        self.to_string().starts_with(prefix.as_ref())
    }

    /// Checks if the string ends with the given suffix.
    ///
    /// # Parameters
    /// - `suffix`: The suffix string to check.
    ///
    /// # Returns
    /// `true` if the string ends with the given suffix, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// assert!(sixbit.ends_with("LO"));
    /// assert!(!sixbit.ends_with("HE"));
    /// ```
    pub fn ends_with<P: AsRef<str>>(&self, suffix: P) -> bool {
        self.to_string().ends_with(suffix.as_ref())
    }

    /// Checks if the string contains the given substring.
    ///
    /// # Parameters
    /// - `substring`: The substring to search for.
    ///
    /// # Returns
    /// `true` if the string contains the given substring, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dec_sixbit::DecSixbit;
    ///
    /// let sixbit = DecSixbit::new("HELLO").unwrap();
    /// assert!(sixbit.contains("ELL"));
    /// assert!(!sixbit.contains("XYZ"));
    /// ```
    pub fn contains<P: AsRef<str>>(&self, substring: P) -> bool {
        self.to_string().contains(substring.as_ref())
    }
}

impl fmt::Display for DecSixbit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use decode_unchecked because the TRAILING_SPACE_MARKER byte might have been added at the end
        let decoded =  decode_unchecked(&self.bytes, self.len);
        write!(f, "{}", decoded)
    }
}

impl std::str::FromStr for DecSixbit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for DecSixbit {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl TryFrom<&[u8]> for DecSixbit {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_slice(bytes)
    }
}

impl TryFrom<Vec<u8>> for DecSixbit {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryFrom<&Vec<u8>> for DecSixbit {
    type Error = Error;

    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl AsRef<[u8]> for DecSixbit {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl serde::Serialize for DecSixbit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            (&self.len, &self.bytes).serialize(serializer)
        }
    }
}

mod deserialize {
    use super::DecSixbit;

    pub(super) struct DecSixbitVisitor;

    #[allow(clippy::needless_lifetimes)]
    impl<'de> serde::de::Visitor<'de> for DecSixbitVisitor {
        type Value = DecSixbit;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("bytes or string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            DecSixbit::new(v).map_err(E::custom)
        }
    }
}

impl<'de> serde::Deserialize<'de> for DecSixbit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<DecSixbit, D::Error> {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            deserializer
                .deserialize_str(deserialize::DecSixbitVisitor)
                .map_err(D::Error::custom)
        } else {
            let (len, bytes) = <(usize, Vec<u8>)>::deserialize(deserializer)?;
            Ok(DecSixbit { len, bytes })
        }
    }
}
