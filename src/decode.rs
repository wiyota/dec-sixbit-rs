//! Functions for decoding DEC SIXBIT-encoded bytes back into strings.
//!
//! This module provides both safe and unsafe decoding functions. The safe functions perform validation
//! to ensure all SIXBIT values are within the valid range, while the unsafe functions assume the input
//! is already valid for increased performance.

use crate::Error;

/// This function converts a slice of SIXBIT-encoded bytes into the original string based on the provided length.
///
/// # Parameters
/// - `bytes`: A slice of bytes containing SIXBIT-encoded data.
/// - `len`: The length of the original string.
///
/// # Errors
/// Returns an [`Error::InvalidSixbitValue`] if any SIXBIT value is outside the valid range (0-63),
/// or if the decoded bytes do not form a valid UTF-8 string.
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
    if len == 0 {
        return Ok(String::new());
    }

    let mut result = Vec::with_capacity(len);

    let full_chunks = len / 4;
    let remaining_chars = len % 4;

    for chunk_idx in 0..full_chunks {
        let byte_idx = chunk_idx * 3;

        let val1 = bytes[byte_idx] >> 2;
        let val2 = ((bytes[byte_idx] & 0b11) << 4) | (bytes[byte_idx + 1] >> 4);
        let val3 = ((bytes[byte_idx + 1] & 0b1111) << 2) | (bytes[byte_idx + 2] >> 6);
        let val4 = bytes[byte_idx + 2] & 0b111111;

        for &val in &[val1, val2, val3, val4] {
            if val > 63 {
                return Err(Error::InvalidSixbitValue(val));
            }
            result.push(val + 32);
        }
    }

    // Handle remaining characters
    if remaining_chars > 0 {
        let start_byte = full_chunks * 3;
        let remaining_bytes = &bytes[start_byte..];

        match remaining_chars {
            1 => {
                let val1 = remaining_bytes[0] >> 2;
                if val1 > 63 {
                    return Err(Error::InvalidSixbitValue(val1));
                }
                result.push(val1 + 32);
            },
            2 => {
                let val1 = remaining_bytes[0] >> 2;
                let val2 = ((remaining_bytes[0] & 0b11) << 4) | (remaining_bytes[1] >> 4);
                if val1 > 63 || val2 > 63 {
                    return Err(Error::InvalidSixbitValue(0));
                }
                result.push(val1 + 32);
                result.push(val2 + 32);
            },
            3 => {
                let val1 = remaining_bytes[0] >> 2;
                let val2 = ((remaining_bytes[0] & 0b11) << 4) | (remaining_bytes[1] >> 4);
                let val3 = ((remaining_bytes[1] & 0b1111) << 2) | (remaining_bytes[2] >> 6);
                if val1 > 63 || val2 > 63 || val3 > 63 {
                    return Err(Error::InvalidSixbitValue(0));
                }
                result.push(val1 + 32);
                result.push(val2 + 32);
                result.push(val3 + 32);
            },
            _ => unreachable!(),
        }
    }

    Ok(String::from_utf8(result).unwrap())
}

/// This function performs decoding without validating whether the SIXBIT values are within the
/// valid range or whether the resulting bytes form a valid UTF-8 string. Use this function only
/// when you are certain the input is valid to avoid undefined behavior.
///
/// # Safety
/// The `bytes` slice must contain valid SIXBIT-encoded data:
/// - All SIXBIT values must be within 0-63.
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

    let mut result = Vec::with_capacity(len);

    let full_chunks = len / 4;
    let remaining_chars = len % 4;

    for chunk_idx in 0..full_chunks {
        let byte_idx = chunk_idx * 3;

        let val1 = bytes[byte_idx] >> 2;
        let val2 = ((bytes[byte_idx] & 0b11) << 4) | (bytes[byte_idx + 1] >> 4);
        let val3 = ((bytes[byte_idx + 1] & 0b1111) << 2) | (bytes[byte_idx + 2] >> 6);
        let val4 = bytes[byte_idx + 2] & 0b111111;

        result.push(val1 + 32);
        result.push(val2 + 32);
        result.push(val3 + 32);
        result.push(val4 + 32);
    }

    // Handle remaining characters
    if remaining_chars > 0 {
        let start_byte = full_chunks * 3;
        let remaining_bytes = &bytes[start_byte..];

        match remaining_chars {
            1 => {
                let val1 = remaining_bytes[0] >> 2;
                result.push(val1 + 32);
            },
            2 => {
                let val1 = remaining_bytes[0] >> 2;
                let val2 = ((remaining_bytes[0] & 0b11) << 4) | (remaining_bytes[1] >> 4);
                result.push(val1 + 32);
                result.push(val2 + 32);
            },
            3 => {
                let val1 = remaining_bytes[0] >> 2;
                let val2 = ((remaining_bytes[0] & 0b11) << 4) | (remaining_bytes[1] >> 4);
                let val3 = ((remaining_bytes[1] & 0b1111) << 2) | (remaining_bytes[2] >> 6);
                result.push(val1 + 32);
                result.push(val2 + 32);
                result.push(val3 + 32);
            },
            _ => unreachable!(),
        }
    }

    // SAFETY: The caller must ensure that all values are valid and will form a valid UTF-8 string
    String::from_utf8_unchecked(result)
}
