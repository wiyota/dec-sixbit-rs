//! Functions for decoding DEC SIXBIT-encoded bytes back into strings.
//!
//! This module provides both safe and unsafe decoding functions. The safe functions perform validation
//! to ensure all SIXBIT values are within the valid range, while the unsafe functions assume the input
//! is already valid for increased performance.

use crate::{Error, ASCII_OFFSET, MASK_FOUR_BITS, MASK_SIX_BITS, MASK_TWO_BITS, SHIFT_FOUR_BITS, SHIFT_SIX_BITS, SHIFT_TWO_BITS};

/// This function converts a slice of SIXBIT-encoded bytes into the original string based on the provided length.
///
/// # Parameters
/// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
/// - `len`: The length of the original string.
///
/// # Errors
/// Returns an [`Error::InvalidBytesLength`] if `bytes.len()` and `len` are inconsistent.
///
/// # Examples
///
/// ```rust
/// use dec_sixbit::{encode, decode};
///
/// let input = "HELLO";
/// let (encoded_bytes, length) = encode(input).unwrap();
/// let decoded_string = decode(&encoded_bytes, length).unwrap();
/// assert_eq!(decoded_string, input);
/// ```
pub fn decode(bytes: &[u8], len: usize) -> Result<String, Error> {
    let bytes_len = bytes.len();

    if bytes_len == 0 {
        if len == 0 {
            return Ok(String::new());
        } else {
            return Err(Error::InvalidBytesLength);
        }
    }

    if bytes_len != (len * 6 + 7) / 8 {
        return Err(Error::InvalidBytesLength);
    }

    Ok(decode_core(bytes, len))
}

/// This function performs decoding without validating whether the SIXBIT values are within the
/// valid range or whether the resulting bytes form a valid UTF-8 string. Use this function only
/// when you are certain the input is valid to avoid undefined behavior.
///
/// # Safety
/// The `bytes` slice must contain valid SIXBIT-encoded data:
/// - The `len` must accurately reflect the number of original characters.
///
/// # Parameters
/// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
/// - `len`: The length of the original string.
///
/// # Returns
/// The decoded string.
///
/// # Examples
///
/// ```rust
/// use dec_sixbit::{encode, decode_unchecked};
///
/// let input = "HELLO";
/// let (encoded_bytes, length) = encode(input).unwrap();
/// let decoded_string = unsafe { decode_unchecked(&encoded_bytes, length) };
/// assert_eq!(decoded_string, input);
/// ```
pub fn decode_unchecked(bytes: &[u8], len: usize) -> String {
    if len == 0 {
        return String::new();
    }

    decode_core(bytes, len)
}

fn decode_core(bytes: &[u8], len: usize) -> String {
    let mut result = Vec::with_capacity(len);
    let full_chunks = len / 4;
    let remaining_chars = len % 4;

    for chunk_idx in 0..full_chunks {
        let byte_idx = chunk_idx * 3;

        push_first_six_bits(bytes, byte_idx, &mut result);
        push_second_six_bits(bytes, byte_idx, &mut result);
        push_third_six_bits(bytes, byte_idx, &mut result);
        push_fourth_six_bits(bytes, byte_idx, &mut result);
    }

    // Handle remaining characters
    if remaining_chars > 0 {
        let start_byte = full_chunks * 3;
        let remaining_bytes = &bytes[start_byte..];

        match remaining_chars {
            1 => {
                push_first_six_bits(remaining_bytes, 0, &mut result);
            },
            2 => {
                push_first_six_bits(remaining_bytes, 0, &mut result);
                push_second_six_bits(remaining_bytes, 0, &mut result);
            },
            3 => {
                push_first_six_bits(remaining_bytes, 0, &mut result);
                push_second_six_bits(remaining_bytes, 0, &mut result);
                push_third_six_bits(remaining_bytes, 0, &mut result);
            },
            _ => unreachable!(),
        }
    }

    // SAFETY: Each byte of result is guaranteed to fit to any ASCII printable character
    unsafe { String::from_utf8_unchecked(result) }
}

#[inline]
fn push_first_six_bits(bytes: &[u8], index: usize, vec: &mut Vec<u8>) {
    let val = bytes[index] >> SHIFT_TWO_BITS;
    vec.push(val + ASCII_OFFSET);
}

#[inline]
fn push_second_six_bits(bytes: &[u8], index: usize, vec: &mut Vec<u8>) {
    let val = ((bytes[index] & MASK_TWO_BITS) << SHIFT_FOUR_BITS) | (bytes[index + 1] >> SHIFT_FOUR_BITS);
    vec.push(val + ASCII_OFFSET);
}

#[inline]
fn push_third_six_bits(bytes: &[u8], index: usize, vec: &mut Vec<u8>) {
    let val = ((bytes[index + 1] & MASK_FOUR_BITS) << SHIFT_TWO_BITS) | (bytes[index + 2] >> SHIFT_SIX_BITS);
    vec.push(val + ASCII_OFFSET);
}

#[inline]
fn push_fourth_six_bits(bytes: &[u8], index: usize, vec: &mut Vec<u8>) {
    let val = bytes[index + 2] & MASK_SIX_BITS;
    vec.push(val + ASCII_OFFSET);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_empty() {
        let bytes = [];
        let decoded = decode(&bytes, 0).unwrap();
        assert_eq!(decoded, "");
    }

    #[test]
    fn test_decode_basic() {
        let input = "HELLO";
        let (encoded_bytes, length) = crate::encode(input).unwrap();
        let decoded = decode(&encoded_bytes, length).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_decode_unchecked() {
        let input = "WORLD";
        let (encoded_bytes, length) = crate::encode(input).unwrap();
        let decoded = decode_unchecked(&encoded_bytes, length);
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_invalid_length() {
        let bytes = [0u8; 2];
        assert!(decode(&bytes, 3).is_err());
    }

    #[test]
    fn test_not_zero_len_but_empty() {
        let bytes = [0u8; 0];
        let decoded = decode(&bytes, 1);
        assert!(decoded.is_err());
    }
}
