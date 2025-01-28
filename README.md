# dec-sixbit-rs

`dec-sixbit` is a Rust crate for encoding and decoding strings using the DEC SIXBIT format. This encoding scheme is historically used in older DEC (Digital Equipment Corporation) systems and represents characters using 6-bit codes, allowing compact storage and transmission of textual data within specific ASCII ranges.

The conversion between DEC SIXBIT and ASCII is very simple and fast, making it suitable for data compression and transfer.

## Table of Contents

- [dec-sixbit-rs](#dec-sixbit-rs)
  - [Table of Contents](#table-of-contents)
  - [Code Specifications](#code-specifications)
    - [DEC SIXBIT Table](#dec-sixbit-table)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Encoding](#encoding)
      - [Unchecked Encoding](#unchecked-encoding)
    - [Decoding](#decoding)
      - [Unchecked Decoding](#unchecked-decoding)
    - [Using the `DecSixbit` Struct API](#using-the-decsixbit-struct-api)
  - [Error Handling](#error-handling)
    - [Example](#example)
  - [Examples](#examples)
    - [Encoding and Decoding](#encoding-and-decoding)
    - [Using the `DecSixbit` Struct](#using-the-decsixbit-struct)
  - [Testing](#testing)
  - [License](#license)

## Code Specifications

DEC SIXBIT, which was commonly used, represents 64 characters using 6 bits each, covering ASCII codes from 32 to 95 while excluding control characters. This includes uppercase alphabet letters, numbers, spaces, and some symbols.

Conversion to DEC SIXBIT is highly efficient as it only involves subtracting 32 from the byte representation of ASCII characters and removing the top 2 bits. This reduces data size by nearly 25% without sacrificing speed, especially in short strings such as user IDs. (21 characters can be stored in 16 bytes)

### DEC SIXBIT Table

|        | 0       | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | A   | B   | C    | D   | E    | F   |
| ------ | ------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- | ---- | --- |
| **0x** | (SPACE) | !   | "   | #   | $   | %   | &   | '   | (   | )   | \*  | +   | ,    | -   | .    | /   |
| **1x** | 0       | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | :   | ;   | &lt; | =   | &gt; | ?   |
| **2x** | @       | A   | B   | C   | D   | E   | F   | G   | H   | I   | J   | K   | L    | M   | N    | O   |
| **3x** | P       | Q   | R   | S   | T   | U   | V   | W   | X   | Y   | Z   | [   | \    | ]   | ^    | \_  |

## Features

- **Efficient Encoding & Decoding**: Convert between standard UTF-8 strings and the compact DEC SIXBIT format.
- **Safety and Performance**: Offers both checked and unchecked encoding/decoding functions for flexibility.
- **Struct API**: Provides a `DecSixbit` struct for a more encapsulated and feature-rich API (enabled via the default `with-struct` feature).
- **Comprehensive Testing**: Includes a suite of tests to ensure reliability and correctness.
- **Error Handling**: Clearly defined error types for invalid input data.

## Installation

Add `dec-sixbit` to your project's `Cargo.toml`:

```toml
[dependencies]
dec-sixbit = "0.1.4" # Replace with the latest version
```

## Usage

### Encoding

To encode a string into DEC SIXBIT format, use the `encode` function:

```rust
fn main() -> Result<(), dec_sixbit::Error> {
    let input = "HELLO";
    let (encoded_bytes, length) = dec_sixbit::encode(input)?;
    println!("Encoded Bytes: {:?}", encoded_bytes);
    println!("Original Length: {}", length);
    Ok(())
}
```

#### Unchecked Encoding

If you can guarantee that all characters are within the valid SIXBIT range (ASCII 32-95), you can use the `encode_unchecked` function for performance gains:

```rust
fn main() {
    let input = "HELLO";
    let (encoded_bytes, length) = dec_sixbit::encode_unchecked(input);
    println!("Encoded Bytes: {:?}", encoded_bytes);
    println!("Original Length: {}", length);
}
```

### Decoding

To decode DEC SIXBIT-encoded bytes back into a string, use the `decode` function:

```rust
fn main() -> Result<(), dec_sixbit::Error> {
    let encoded_bytes = vec![0b10000110, 0b00101000, 0b11100100];
    let length = 4;
    let decoded_string = dec_sixbit::decode(&encoded_bytes, length)?;
    println!("Decoded String: {}", decoded_string);
    Ok(())
}
```

#### Unchecked Decoding

For scenarios where you are certain the encoded bytes are valid, use the `decode_unchecked` function:

```rust
fn main() {
    let encoded_bytes = vec![0b10000110, 0b00101000, 0b11100100];
    let length = 4;
    let decoded_string = dec_sixbit::decode_unchecked(&encoded_bytes, length);
    println!("Decoded String: {}", decoded_string);
}
```

### Using the `DecSixbit` Struct API

The `DecSixbit` struct provides a more structured approach to handling SIXBIT-encoded data. This API is available when the default `with-struct` feature is enabled.

```rust
use dec_sixbit::DecSixbit;

fn main() -> Result<(), dec_sixbit::Error> {
    let original = "HELLO";
    let sixbit = DecSixbit::new(original)?;

    println!("Encoded Bytes: {:?}", sixbit.as_bytes());
    println!("Original Length: {}", sixbit.len());

    // Display as string
    println!("Decoded String: {}", sixbit);

    Ok(())
}
```

## Error Handling

`dec-sixbit` defines a custom `Error` enum to handle various error scenarios:

- `InvalidCharacter`: Triggered when the input string contains characters outside the valid SIXBIT range (ASCII 32-95).
- `InvalidBytesLength`: Occurs when decoding encounters iconsistent byte length and string length.

### Example

```rust
fn main() {
    match dec_sixbit::encode("Hello!") {
        Ok((bytes, len)) => {
            println!("Encoded Bytes: {:?}", bytes);
            println!("Length: {}", len);
        },
        Err(e) => {
            eprintln!("Encoding failed: {}", e);
        },
    }
}
```

## Examples

### Encoding and Decoding

```rust
fn main() -> Result<(), dec_sixbit::Error> {
    let original = "DEC SIXBIT EXAMPLE!";

    // Encode
    let (encoded_bytes, length) = dec_sixbit::encode(original)?;
    println!("Encoded Bytes: {:?}", encoded_bytes);
    println!("Original Length: {}", length);

    // Decode
    let decoded = dec_sixbit::decode(&encoded_bytes, length)?;
    println!("Decoded String: {}", decoded);

    assert_eq!(original, decoded);
    Ok(())
}
```

### Using the `DecSixbit` Struct

```rust
use dec_sixbit::DecSixbit;

fn main() -> Result<(), dec_sixbit::Error> {
    let original = "STRUCTURE API EXAMPLE";
    let sixbit = DecSixbit::new(original)?;

    println!("Encoded Bytes: {:?}", sixbit.as_bytes());
    println!("Original Length: {}", sixbit.len());
    println!("Decoded String: {}", sixbit);

    Ok(())
}
```

## Testing

`dec-sixbit` includes a comprehensive test suite to ensure functionality and reliability. To run the tests, navigate to the project directory and execute:

```sh
cargo test
```

This will run all unit tests defined in the library, covering encoding, decoding, error handling, and the struct API.

## License

This project is licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT License](./LICENSE-MIT) at your option.

---

Feel free to contribute to `dec-sixbit` by submitting issues or pull requests on [GitHub](https://github.com/wiyota/dec-sixbit-rs).
