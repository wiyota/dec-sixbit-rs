//! Functions for encoding strings into DEC SIXBIT format.
//!
//! This module provides both safe and unsafe encoding functions. The safe functions perform validation
//! to ensure all characters are within the valid SIXBIT range, while the unsafe functions assume the input
//! is already valid for increased performance.

use crate::{Error, MASK_FOUR_BITS, MASK_TWO_BITS, ASCII_OFFSET, SHIFT_TWO_BITS, SHIFT_FOUR_BITS, SHIFT_SIX_BITS};

/// This function converts the input string into a compact SIXBIT-encoded byte vector and returns the
/// encoded bytes along with the original string length.
///
/// # Constraints
/// - Only ASCII characters in the range 32-95 (space through underscore) are allowed.
///
/// # Errors
/// Returns an [`Error::InvalidCharacter`] if the input contains characters outside the valid range.
///
/// # Examples
///
/// ```rust
/// use dec_sixbit::encode;
///
/// let input = "HELLO";
/// let (encoded_bytes, length) = encode(input).unwrap();
/// ```
pub fn encode(str: &str) -> Result<(Vec<u8>, usize), Error> {
    // Check if input string contains only ASCII characters
    if !str.is_ascii() {
        return Err(Error::InvalidCharacter);
    }
    let len = str.len();
    // Every 4 characters need 3 bytes, round up
    let bytes_needed = (len * 3 + 3) / 4;
    let mut bytes = vec![0u8; bytes_needed];

    let full_chunks = len / 4;
    let remaining = len % 4;

    for chunk_idx in 0..full_chunks {
        let start = chunk_idx * 4;
        let chunk = &str.as_bytes()[start..start + 4];

        // Validate characters
        for &code in chunk {
            if !(ASCII_OFFSET..=95).contains(&code) {
                return Err(Error::InvalidCharacter);
            }
        }

        // Convert to SIXBIT values by subtracting ASCII_OFFSET
        let a = chunk[0] - ASCII_OFFSET;
        let b = chunk[1] - ASCII_OFFSET;
        let c = chunk[2] - ASCII_OFFSET;
        let d = chunk[3] - ASCII_OFFSET;

        let byte_idx = chunk_idx * 3;

        // Pack 4 SIXBIT values into 3 bytes
        bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
        bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
        bytes[byte_idx + 2] = ((c & MASK_TWO_BITS) << SHIFT_SIX_BITS) | d;
    }

    // Handle the remaining 1-3 characters, if any
    if remaining > 0 {
        let start = full_chunks * 4;
        let chunk = &str.as_bytes()[start..];
        let byte_idx = full_chunks * 3;

        match chunk.len() {
            3 => {
                // Validate characters
                for &code in chunk {
                    if !(ASCII_OFFSET..=95).contains(&code) {
                        return Err(Error::InvalidCharacter);
                    }
                }

                // Convert to SIXBIT values by subtracting ASCII_OFFSET
                let a = chunk[0] - ASCII_OFFSET;
                let b = chunk[1] - ASCII_OFFSET;
                let c = chunk[2] - ASCII_OFFSET;

                // Pack 3 SIXBIT values into 2.25 bytes (rounded up to 3 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
                bytes[byte_idx + 2] = (c & MASK_TWO_BITS) << SHIFT_SIX_BITS;
            },
            2 => {
                // Validate characters
                for &code in chunk {
                    if !(ASCII_OFFSET..=95).contains(&code) {
                        return Err(Error::InvalidCharacter);
                    }
                }

                // Convert to SIXBIT values by subtracting ASCII_OFFSET
                let a = chunk[0] - ASCII_OFFSET;
                let b = chunk[1] - ASCII_OFFSET;

                // Pack 2 SIXBIT values into 1.5 bytes (rounded up to 2 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = (b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS;
            },
            1 => {
                // Validate character
                let code = chunk[0];
                if !(ASCII_OFFSET..=95).contains(&code) {
                    return Err(Error::InvalidCharacter);
                }

                // Convert to SIXBIT value by subtracting ASCII_OFFSET
                let a = code - ASCII_OFFSET;

                // Pack 1 SIXBIT value into 0.75 bytes (rounded up to 1 byte)
                bytes[byte_idx] = a << SHIFT_TWO_BITS;
            },
            _ => unreachable!(),
        }
    }

    Ok((bytes, len))
}

/// This function performs encoding without validating whether the input string contains only
/// valid SIXBIT characters (ASCII 32-95). Use this function only when you are certain the input
/// meets the required constraints to avoid undefined behavior.
///
/// # Safety
/// The caller must ensure that all characters in `str` are within the valid SIXBIT range (32-95).
///
/// # Examples
///
/// ```rust
/// use dec_sixbit::encode_unchecked;
///
/// let input = "HELLO";
/// let (encoded_bytes, length) = unsafe { encode_unchecked(input) };
/// ```
pub fn encode_unchecked(str: &str) -> (Vec<u8>, usize) {
    let len = str.len();
    // Every 4 characters need 3 bytes, round up
    let bytes_needed = (len * 3 + 3) / 4;
    let mut bytes = vec![0u8; bytes_needed];

    let full_chunks = len / 4;
    let remaining = len % 4;

    for chunk_idx in 0..full_chunks {
        let start = chunk_idx * 4;
        let chunk = &str.as_bytes()[start..start + 4];

        // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
        let a = chunk[0] - ASCII_OFFSET;
        let b = chunk[1] - ASCII_OFFSET;
        let c = chunk[2] - ASCII_OFFSET;
        let d = chunk[3] - ASCII_OFFSET;

        let byte_idx = chunk_idx * 3;

        // Pack 4 SIXBIT values into 3 bytes
        bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
        bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
        bytes[byte_idx + 2] = ((c & MASK_TWO_BITS) << SHIFT_SIX_BITS) | d;
    }

    // Handle the remaining 1-3 characters, if any
    if remaining > 0 {
        let start = full_chunks * 4;
        let chunk = &str.as_bytes()[start..];
        let byte_idx = full_chunks * 3;

        match chunk.len() {
            3 => {
                // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
                let a = chunk[0] - ASCII_OFFSET;
                let b = chunk[1] - ASCII_OFFSET;
                let c = chunk[2] - ASCII_OFFSET;

                // Pack 3 SIXBIT values into 2.25 bytes (rounded up to 3 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS);
                bytes[byte_idx + 2] = (c & MASK_TWO_BITS) << SHIFT_SIX_BITS;
            },
            2 => {
                // Convert to SIXBIT values by subtracting ASCII_OFFSET directly
                let a = chunk[0] - ASCII_OFFSET;
                let b = chunk[1] - ASCII_OFFSET;

                // Pack 2 SIXBIT values into 1.5 bytes (rounded up to 2 bytes)
                bytes[byte_idx] = (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS);
                bytes[byte_idx + 1] = (b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS;
            },
            1 => {
                // Convert to SIXBIT value by subtracting ASCII_OFFSET directly
                let a = chunk[0] - ASCII_OFFSET;

                // Pack 1 SIXBIT value into 0.75 bytes (rounded up to 1 byte)
                bytes[byte_idx] = a << SHIFT_TWO_BITS;
            },
            _ => unreachable!(),
        }
    }

    (bytes, len)
}
