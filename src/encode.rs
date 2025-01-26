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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty_string() {
        let input = "";
        let (encoded, len) = encode(input).expect("Encoding should succeed for empty string");
        assert!(encoded.is_empty(), "Encoded bytes should be empty");
        assert_eq!(len, 0, "Length should be 0");
    }

    #[test]
    fn test_encode_single_character() {
        let input = "A"; // ASCII 65
        let (encoded, len) = encode(input).expect("Encoding should succeed for single character");
        let expected = vec![(65 - ASCII_OFFSET) << SHIFT_TWO_BITS];
        assert_eq!(encoded, expected, "Encoded bytes do not match expected value");
        assert_eq!(len, 1, "Length should be 1");
    }

    #[test]
    fn test_encode_two_characters() {
        let input = "AB"; // ASCII 65, 66
        let (encoded, len) = encode(input).expect("Encoding should succeed for two characters");
        let a = 65 - ASCII_OFFSET;
        let b = 66 - ASCII_OFFSET;
        let expected = vec![
            (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS),
            (b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS,
        ];
        assert_eq!(encoded, expected, "Encoded bytes do not match expected value for two characters");
        assert_eq!(len, 2, "Length should be 2");
    }

    #[test]
    fn test_encode_three_characters() {
        let input = "ABC"; // ASCII 65, 66, 67
        let (encoded, len) = encode(input).expect("Encoding should succeed for three characters");
        let a = 65 - ASCII_OFFSET;
        let b = 66 - ASCII_OFFSET;
        let c = 67 - ASCII_OFFSET;
        let expected = vec![
            (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS),
            ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS),
            (c & MASK_TWO_BITS) << SHIFT_SIX_BITS,
        ];
        assert_eq!(encoded, expected, "Encoded bytes do not match expected value for three characters");
        assert_eq!(len, 3, "Length should be 3");
    }

    #[test]
    fn test_encode_four_characters() {
        let input = "ABCD"; // ASCII 65, 66, 67, 68
        let (encoded, len) = encode(input).expect("Encoding should succeed for four characters");
        let a = 65 - ASCII_OFFSET;
        let b = 66 - ASCII_OFFSET;
        let c = 67 - ASCII_OFFSET;
        let d = 68 - ASCII_OFFSET;
        let expected = vec![
            (a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS),
            ((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS),
            ((c & MASK_TWO_BITS) << SHIFT_SIX_BITS) | d,
        ];
        assert_eq!(encoded, expected, "Encoded bytes do not match expected value for four characters");
        assert_eq!(len, 4, "Length should be 4");
    }

    #[test]
    fn test_encode_multiple_chunks() {
        let input = "HELLOWORLD_ "; // 12 characters
        let (encoded, len) = encode(input).expect("Encoding should succeed for multiple chunks");
        assert_eq!(len, input.len(), "Length should match input length");

        // Manually compute expected bytes
        // Chunks: "HELL", "OWOR", "LD_ "
        let chunks = ["HELL", "OWOR", "LD_ "];
        let mut expected = Vec::new();

        for chunk in &chunks {
            let a = chunk.as_bytes()[0] - ASCII_OFFSET;
            let b = chunk.as_bytes()[1] - ASCII_OFFSET;
            let c = chunk.as_bytes()[2] - ASCII_OFFSET;
            let d = chunk.as_bytes()[3] - ASCII_OFFSET;

            expected.push((a << SHIFT_TWO_BITS) | (b >> SHIFT_FOUR_BITS));
            expected.push(((b & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | (c >> SHIFT_TWO_BITS));
            expected.push(((c & MASK_TWO_BITS) << SHIFT_SIX_BITS) | d);
        }

        assert_eq!(encoded, expected, "Encoded bytes do not match expected value for multiple chunks");
    }

    #[test]
    fn test_encode_with_invalid_character_non_ascii() {
        let input = "Hello€"; // '€' is not ASCII
        let result = encode(input);
        assert!(matches!(result, Err(Error::InvalidCharacter)), "Should return InvalidCharacter error for non-ASCII characters");
    }

    #[test]
    fn test_encode_with_invalid_character_below_range() {
        let input = "HELLO\x1F"; // ASCII 31, below valid range
        let result = encode(input);
        assert!(matches!(result, Err(Error::InvalidCharacter)), "Should return InvalidCharacter error for characters below range");
    }

    #[test]
    fn test_encode_with_invalid_character_above_range() {
        let input = "HELLO~"; // '~' is ASCII 126, above valid range
        let result = encode(input);
        assert!(matches!(result, Err(Error::InvalidCharacter)), "Should return InvalidCharacter error for characters above range");
    }

    #[test]
    fn test_encode_unchecked_valid_input() {
        let input = "ABCD";
        let (safe_encoded, _) = encode(input).expect("Safe encode should succeed for valid input");
        let (unsafe_encoded, _) = encode_unchecked(input);
        assert_eq!(safe_encoded, unsafe_encoded, "Unchecked encoding should match safe encoding for valid input");
    }

    #[test]
    fn test_encode_unchecked_empty_string() {
        let input = "";
        let (encoded, len) = encode_unchecked(input);
        assert!(encoded.is_empty(), "Encoded bytes should be empty for empty string");
        assert_eq!(len, 0, "Length should be 0 for empty string");
    }

    #[test]
    fn test_encode_unchecked_large_input() {
        let input = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG_12345";
        let (safe_encoded, len_safe) = encode(input).expect("Safe encode should succeed for large input");
        let (unsafe_encoded, len_unsafe) = {
            let (bytes, len) = encode_unchecked(input);
            (bytes, len)
        };
        assert_eq!(safe_encoded, unsafe_encoded, "Unchecked encoding should match safe encoding for large input");
        assert_eq!(len_safe, len_unsafe, "Lengths should match for large input");
    }

    #[test]
    #[allow(clippy::precedence)]
    fn test_encode_partial_chunks() {
        // Test inputs that result in partial chunks (1-3 remaining characters)
        let cases = vec![
            ("A", vec![(65 - ASCII_OFFSET) << SHIFT_TWO_BITS], 1),
            ("AB", vec![(65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS), ((66 - ASCII_OFFSET) & 0b1111) << SHIFT_FOUR_BITS], 2),
            ("ABC", vec![
                (65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS),
                (((66 - ASCII_OFFSET) & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | ((67 - ASCII_OFFSET) >> SHIFT_TWO_BITS),
                ((67 - ASCII_OFFSET) & MASK_TWO_BITS) << SHIFT_SIX_BITS
            ], 3),
            ("ABCDE", vec![
                // "ABCD"
                (65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS),
                (((66 - ASCII_OFFSET) & MASK_FOUR_BITS) << SHIFT_FOUR_BITS) | ((67 - ASCII_OFFSET) >> SHIFT_TWO_BITS),
                ((67 - ASCII_OFFSET) & MASK_TWO_BITS) << SHIFT_SIX_BITS | (68 - ASCII_OFFSET),
                // "E"
                (69 - ASCII_OFFSET) << SHIFT_TWO_BITS
            ], 5),
        ];

        for (input, expected, len) in cases {
            let (encoded, encoded_len) = encode(input).expect("Encoding should succeed");
            assert_eq!(encoded, expected, "Encoded bytes do not match for input '{}'", input);
            assert_eq!(encoded_len, len, "Length does not match for input '{}'", input);
        }
    }

    #[test]
    fn test_encode_unchecked_two_characters() {
        let input = "AB"; // ASCII 65, 66
        // Confirm that encode_unchecked produces the same result after safely encoding
        let (safe_encoded, _) = encode(input).expect("Safe encode should succeed for two characters");
        let (unsafe_encoded, _) = encode_unchecked(input);
        assert_eq!(safe_encoded, unsafe_encoded, "Unchecked encoding should match safe encoding for two characters");
    }

    #[test]
    fn test_encode_unchecked_three_characters() {
        let input = "ABC"; // ASCII 65, 66, 67
        // Confirm that encode_unchecked produces the same result after safely encoding
        let (safe_encoded, _) = encode(input).expect("Safe encode should succeed for three characters");
        let (unsafe_encoded, _) = encode_unchecked(input);
        assert_eq!(safe_encoded, unsafe_encoded, "Unchecked encoding should match safe encoding for three characters");
    }

    #[test]
    #[allow(clippy::precedence)]
    fn test_encode_unchecked_partial_chunks() {
        // Cases including partial chunks (1-3 characters)
        let cases = vec![
            ("A", vec![(65 - ASCII_OFFSET) << SHIFT_TWO_BITS], 1),
            ("AB", vec![
                (65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS),
                ((66 - ASCII_OFFSET) & MASK_FOUR_BITS) << SHIFT_FOUR_BITS,
            ], 2),
            ("ABC", vec![
                (65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS),
                ((66 - ASCII_OFFSET) & MASK_FOUR_BITS) << SHIFT_FOUR_BITS | ((67 - ASCII_OFFSET) >> SHIFT_TWO_BITS),
                ((67 - ASCII_OFFSET) & MASK_TWO_BITS) << SHIFT_SIX_BITS,
            ], 3),
            ("ABCDE", vec![
                // "ABCD"
                (65 - ASCII_OFFSET) << SHIFT_TWO_BITS | ((66 - ASCII_OFFSET) >> SHIFT_FOUR_BITS),
                ((66 - ASCII_OFFSET) & MASK_FOUR_BITS) << SHIFT_FOUR_BITS | ((67 - ASCII_OFFSET) >> SHIFT_TWO_BITS),
                ((67 - ASCII_OFFSET) & MASK_TWO_BITS) << SHIFT_SIX_BITS | (68 - ASCII_OFFSET),
                // "E"
                (69 - ASCII_OFFSET) << SHIFT_TWO_BITS,
            ], 5),
        ];

        for (input, expected, len) in cases {
            let (safe_encoded, encoded_len_safe) = encode(input).expect("Safe encode should succeed");
            let (unsafe_encoded, encoded_len_unsafe) = encode_unchecked(input);
            assert_eq!(safe_encoded, expected, "Safe encoding does not match expected for input '{}'", input);
            assert_eq!(unsafe_encoded, expected, "Unchecked encoding does not match expected for input '{}'", input);
            assert_eq!(encoded_len_safe, len, "Length does not match expected value for input '{}'", input);
            assert_eq!(encoded_len_unsafe, len, "Length should be correct for input '{}'", input);
        }
    }
}
