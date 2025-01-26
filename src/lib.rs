#![doc = include_str!("../README.md")]

mod encode;
mod decode;
#[cfg(feature = "with-struct")]
mod struct_api;

pub use encode::{encode, encode_unchecked};
pub use decode::{decode, decode_unchecked};
#[cfg(feature = "with-struct")]
pub use struct_api::DecSixbit;

const MASK_TWO_BITS: u8 = 0b11;
const MASK_FOUR_BITS: u8 = 0b1111;
const MASK_SIX_BITS: u8 = 0b111111;
const SHIFT_TWO_BITS: u8 = 2;
const SHIFT_FOUR_BITS: u8 = 4;
const SHIFT_SIX_BITS: u8 = 6;
const ASCII_OFFSET: u8 = 32;

/// Represents errors that can occur during encoding or decoding operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum Error {
    /// Occurs when the input string contains a character outside the valid SIXBIT range (ASCII 32-95).
    #[error("invalid character in input (must be ASCII 32-95)")]
    InvalidCharacter,

    /// Occurs when decoding fails due to inconsistent input bytes and length.
    #[error("input bytes and length are inconsistent")]
    InvalidBytesLength,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "with-struct")]
    use super::DecSixbit;

    #[test]
    fn test_packed_storage() {
        let input = "ABCD"; // 4 chars should pack into 3 bytes

        // Calculate expected values
        let a = 65 - 32; // = 33 = 0b100001
        let b = 66 - 32; // = 34 = 0b100010
        let c = 67 - 32; // = 35 = 0b100011
        let d = 68 - 32; // = 36 = 0b100100

        use std::io::Write;
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();

        writeln!(handle, "Input SIXBIT values:").unwrap();
        writeln!(handle, "  A: dec={} bin={:06b}", a, a).unwrap();
        writeln!(handle, "  B: dec={} bin={:06b}", b, b).unwrap();
        writeln!(handle, "  C: dec={} bin={:06b}", c, c).unwrap();
        writeln!(handle, "  D: dec={} bin={:06b}", d, d).unwrap();

        // Correct expected bytes based on encode_core logic
        let expected = vec![0b10000110, 0b00101000, 0b11100100];
        writeln!(handle, "\nExpected packed bytes:").unwrap();
        writeln!(handle, "  byte 1 = {:08b} = (A<<2 | B>>4)", expected[0]).unwrap();
        writeln!(handle, "  byte 2 = {:08b} = (B<<4 | C>>2)", expected[1]).unwrap();
        writeln!(handle, "  byte 3 = {:08b} = (C<<6 | D)", expected[2]).unwrap();

        // Encapsulated API test
        #[cfg(feature = "with-struct")]
        {
            let sixbit = DecSixbit::new(input).unwrap();
            writeln!(handle, "\nActual packed bytes:  ").unwrap();
            writeln!(handle, "  byte 1 = {:08b}", sixbit.bytes[0]).unwrap();
            writeln!(handle, "  byte 2 = {:08b}", sixbit.bytes[1]).unwrap();
            writeln!(handle, "  byte 3 = {:08b}", sixbit.bytes[2]).unwrap();

            assert_eq!(sixbit.bytes, expected);
            assert_eq!(sixbit.len, 4);
        }
    }

    #[test]
    fn test_partial_packing() {
        let inputs = ["A", "AB", "ABC"];
        let expected_bytes = [
            vec![0b10000100], // "A" packed: [132]
            vec![0b10000110, 0b00100000], // "AB" packed: [134, 32]
            vec![0b10000110, 0b00101000, 0b11000000], // "ABC" packed: [134, 40, 192]
        ];

        for (input, expected) in inputs.iter().zip(expected_bytes.iter()) {
            let (bytes, len) = encode(input).unwrap();
            println!("\nTesting input: '{}' (length {})", input, input.len());

            // Display SIXBIT values
            let values: Vec<u8> = input.bytes().map(|b| b - 32).collect();
            print!("SIXBIT values: ");
            for b in &values {
                print!("{:02}={:06b} ", b, b);
            }
            println!("\nExpected bytes:");
            for (i, &b) in expected.iter().enumerate() {
                println!("  byte {} = {:08b} ({})", i + 1, b, b);
            }

            // Display actual encoded bytes
            println!("Got bytes:");
            for (i, &b) in bytes.iter().enumerate() {
                println!("  byte {} = {:08b} ({})", i + 1, b, b);
            }

            // Assertions
            assert_eq!(bytes, *expected, "Mismatch in encoded bytes for input '{}'", input);
            assert_eq!(len, input.len(), "Mismatch in length for input '{}'", input);
        }
    }

    #[test]
    fn test_encoding_decoding() {
        let inputs = [
            "HELLO WORLD",
            "TEST 123",
            " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_",
        ];

        for input in inputs {
            let (bytes, len) = encode(input).unwrap();
            assert_eq!(len, input.len());

            let decoded = decode(&bytes, len).unwrap();
            assert_eq!(decoded, input);

            let decoded_unchecked = decode_unchecked(&bytes, len);
            assert_eq!(decoded_unchecked, input);

            #[cfg(feature = "with-struct")]
            {
                let sixbit = DecSixbit::new(input).unwrap();
                assert_eq!(sixbit.bytes, bytes);
                assert_eq!(sixbit.len, len);
                assert_eq!(sixbit.to_string(), decoded);
            }
        }
    }

    #[test]
    fn test_invalid_characters() {
        // Test character below range
        assert!(matches!(
            encode("\x1F"),
            Err(Error::InvalidCharacter)
        ));

        // Test character above range
        assert!(matches!(
            encode("abc"),
            Err(Error::InvalidCharacter)
        ));

        // Test non-ASCII character
        assert!(matches!(
            encode("こんにちは"),
            Err(Error::InvalidCharacter)
        ));
    }

    #[test]
    fn test_empty_string() {
        let (bytes, len) = encode("").unwrap();
        assert!(len == 0);
        assert!(bytes.is_empty());

        let decoded = decode(&bytes, len).unwrap();
        assert_eq!(decoded, "");

        let decoded_unchecked = decode_unchecked(&bytes, len);
        assert_eq!(decoded_unchecked, "");

        #[cfg(feature = "with-struct")]
        {
            let sixbit = DecSixbit::new("").unwrap();
            assert!(sixbit.is_empty());
            assert_eq!(sixbit.len, 0);
            assert!(sixbit.as_bytes().is_empty());

            let decoded = decode(&sixbit.bytes, sixbit.len).unwrap();
            assert_eq!(decoded, "");

            let decoded_unchecked = decode_unchecked(&sixbit.bytes, sixbit.len);
            assert_eq!(decoded_unchecked, "");
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_default() {
        {
            let sixbit = DecSixbit::default();
            assert!(sixbit.is_empty());
            assert_eq!(sixbit.len, 0);
            assert!(sixbit.as_bytes().is_empty());
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_as_ref() {
        {
            let input = "TEST";
            let sixbit = DecSixbit::new(input).unwrap();
            let bytes: &[u8] = sixbit.as_ref();
            assert_eq!(bytes, sixbit.as_bytes());
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_try_from() {
        {
            let input = "TEST";
            let sixbit = DecSixbit::try_from(input).unwrap();
            assert_eq!(sixbit.to_string(), input);

            assert!(DecSixbit::try_from("invalid❌").is_err());
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_from_str() {
        {
            let input = "TEST";
            let sixbit: DecSixbit = input.parse().unwrap();
            assert_eq!(sixbit.to_string(), input);

            let result: Result<DecSixbit, _> = "invalid❌".parse();
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_decode_unchecked_integrity() {
        let input = "OPTIMIZATION TEST";
        let (bytes, len) = encode(input).unwrap();
        let decoded = decode(&bytes, len).unwrap();
        assert_eq!(decoded, input);

        let decoded_unchecked = decode_unchecked(&bytes, len);
        assert_eq!(decoded_unchecked, input);

        #[cfg(feature = "with-struct")]
        {
            let sixbit = DecSixbit::new(input).unwrap();
            let decoded = decode::decode(&sixbit.bytes, sixbit.len).unwrap();
            assert_eq!(decoded, input);

            let decoded_unchecked = decode_unchecked(&sixbit.bytes, sixbit.len);
            assert_eq!(decoded_unchecked, input);
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_serde_serialization_readable() {
        let input = "TEST SERIALIZATION";
        let sixbit = DecSixbit::new(input).unwrap();

        // Serialize to JSON (readable format)
        let serialized = serde_json::to_string(&sixbit).expect("Failed to serialize to JSON");
        println!("Serialized JSON: {}", serialized);

        // Deserialize back
        let deserialized: DecSixbit =
            serde_json::from_str(&serialized).expect("Failed to deserialize from JSON");

        assert_eq!(sixbit, deserialized);
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_serde_deserialization_binary() {
        use bincode::Options;
        let my_options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();

        let input = "TEST BINARY DESERIALIZATION";
        let sixbit = DecSixbit::new(input).unwrap();

        // Serialize to binary using bincode
        let serialized = my_options.serialize(&sixbit).expect("Failed to serialize with bincode");
        println!("Serialized binary: {:?}", serialized);

        // Deserialize from binary
        let deserialized: DecSixbit =
            my_options.deserialize(&serialized).expect("Failed to deserialize from bincode");

        assert_eq!(sixbit, deserialized);
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_serde_readable_and_binary() {
        let input = "COMPLEX SERIALIZATION TEST";
        let sixbit = DecSixbit::new(input).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&sixbit).expect("JSON serialization failed");

        // Deserialize from JSON
        let from_json: DecSixbit =
            serde_json::from_str(&json).expect("JSON deserialization failed");
        assert_eq!(sixbit, from_json);

        // Serialize to binary
        let binary = bincode::serialize(&sixbit).expect("Bincode serialization failed");

        // Deserialize from binary
        let from_binary: DecSixbit =
            bincode::deserialize(&binary).expect("Bincode deserialization failed");
        assert_eq!(sixbit, from_binary);
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_serde_roundtrip() {
        let inputs = [
            "",
            "HELLO",
            "WORLD!",
            "SERDE SERIALIZATION TEST 123",
            "SPECIAL CHARS: !\"#$%&'()*+,-./:;<=>?@[]^_[]",
        ];

        for input in &inputs {
            let sixbit = DecSixbit::new(input).expect("Failed to create DecSixbit");

            // Serialize to JSON
            let json = serde_json::to_string(&sixbit).expect("JSON serialization failed");

            // Deserialize from JSON
            let deserialized_json: DecSixbit =
                serde_json::from_str(&json).expect("JSON deserialization failed");
            assert_eq!(sixbit, deserialized_json, "JSON roundtrip failed for input '{}'", input);

            // Serialize to binary
            let binary = bincode::serialize(&sixbit).expect("Bincode serialization failed");

            // Deserialize from binary
            let deserialized_binary: DecSixbit =
                bincode::deserialize(&binary).expect("Bincode deserialization failed");
            assert_eq!(sixbit, deserialized_binary, "Binary roundtrip failed for input '{}'", input);
        }
    }

    #[cfg(feature = "with-struct")]
    #[test]
    fn test_trailing_spaces() {
        let input = "TESTTEST";
        let sixbit = DecSixbit::new(input).unwrap();
        assert_eq!(sixbit.to_string(), "TESTTEST");
        assert_eq!(sixbit.as_bytes().len(), 6);

        let input = "TEST    ";
        let sixbit = DecSixbit::new(input).unwrap();
        assert_eq!(sixbit.to_string(), "TEST    ");
        // The last byte contains DecSixbit::TRAILING_SPACE_MARKER
        assert_eq!(sixbit.as_bytes().len(), 7);
    }
}
