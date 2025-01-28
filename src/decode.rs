//! Functions for decoding DEC SIXBIT-encoded bytes back into strings.
//!
//! This module provides both checked and unchecked decoding functions. The safe functions perform validation
//! to ensure all SIXBIT values are within the valid range, while the unchecked functions assume the input
//! is already valid for increased performance.

use crate::{Error, ASCII_OFFSET, MASK_SIX_BITS};

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
#[inline(always)]
pub fn decode(bytes: &[u8], len: usize) -> Result<String, Error> {
    if bytes.len() != (len * 6 + 7) / 8 {
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
/// let decoded_string = decode_unchecked(&encoded_bytes, length);
/// assert_eq!(decoded_string, input);
/// ```
#[inline(always)]
pub fn decode_unchecked(bytes: &[u8], len: usize) -> String {
    decode_core(bytes, len)
}

#[inline(always)]
fn decode_core(bytes: &[u8], len: usize) -> String {
    if len == 0 {
        return String::new();
    }

    let mut result = vec![0u8; len];
    let full_chunks = len / 4;
    let remaining_chars = len % 4;

    let bytes_ptr = bytes.as_ptr();
    let result_ptr: *mut u8 = result.as_mut_ptr();

    unsafe {
        // Process full chunks
        for chunk_idx in 0..full_chunks {
            let byte_idx = chunk_idx * 3;
            let str_idx = chunk_idx * 4;

            // Load 3 bytes into a 32-bit integer and perform bit operations in a single step
            let bytes = ((*bytes_ptr.add(byte_idx) as u32) << 16)
                      | ((*bytes_ptr.add(byte_idx + 1) as u32) << 8)
                      | (*bytes_ptr.add(byte_idx + 2) as u32);

            // Extract 6-bit values and add ASCII offset in one operation per byte
            let char1 = ((bytes >> 18) as u8 & MASK_SIX_BITS) + ASCII_OFFSET;
            let char2 = ((bytes >> 12) as u8 & MASK_SIX_BITS) + ASCII_OFFSET;
            let char3 = ((bytes >> 6) as u8 & MASK_SIX_BITS) + ASCII_OFFSET;
            let char4 = (bytes as u8 & MASK_SIX_BITS) + ASCII_OFFSET;

            // Store results with sequential memory access
            *result_ptr.add(str_idx) = char1;
            *result_ptr.add(str_idx + 1) = char2;
            *result_ptr.add(str_idx + 2) = char3;
            *result_ptr.add(str_idx + 3) = char4;
        }

        // Process remaining characters
        match remaining_chars {
            0 => {},
            1 => {
                let byte0 = *bytes_ptr.add(full_chunks * 3);
                let char1 = (byte0 >> 2) + ASCII_OFFSET;
                *result_ptr.add(full_chunks * 4) = char1;
            },
            2 => {
                let byte0 = *bytes_ptr.add(full_chunks * 3);
                let byte1 = *bytes_ptr.add(full_chunks * 3 + 1);
                let char1 = (byte0 >> 2) + ASCII_OFFSET;
                let char2 = (((byte0 & 0b00000011) << 4) | (byte1 >> 4)) + ASCII_OFFSET;
                *result_ptr.add(full_chunks * 4) = char1;
                *result_ptr.add(full_chunks * 4 + 1) = char2;
            },
            3 => {
                let byte0 = *bytes_ptr.add(full_chunks * 3);
                let byte1 = *bytes_ptr.add(full_chunks * 3 + 1);
                let byte2 = *bytes_ptr.add(full_chunks * 3 + 2);
                let char1 = (byte0 >> 2) + ASCII_OFFSET;
                let char2 = (((byte0 & 0b00000011) << 4) | (byte1 >> 4)) + ASCII_OFFSET;
                let char3 = (((byte1 & 0b00001111) << 2) | (byte2 >> 6)) + ASCII_OFFSET;
                *result_ptr.add(full_chunks * 4) = char1;
                *result_ptr.add(full_chunks * 4 + 1) = char2;
                *result_ptr.add(full_chunks * 4 + 2) = char3;
            },
            _ => unreachable!(),
        }
    }

    // SAFETY: Each byte of result is guaranteed to fit to any ASCII printable character
    unsafe { String::from_utf8_unchecked(result) }
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
