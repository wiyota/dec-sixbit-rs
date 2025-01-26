# dec-sixbit-rs

`dec-sixbit` is a Rust crate for encoding and decoding strings using the DEC SIXBIT format. This encoding scheme is historically used in older DEC (Digital Equipment Corporation) systems and represents characters using 6-bit codes, allowing compact storage and transmission of textual data within specific ASCII ranges.

The conversion between DEC SIXBIT and ASCII is very simple and fast, making it suitable for data compression and transfer.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)

  - [Encoding](#encoding)
  - [Decoding](#decoding)
  - [Using the `DecSixbit` Struct API](#using-the-decsixbit-struct-api)

- [Error Handling](#error-handling)
- [Examples](#examples)
- [Testing](#testing)
- [License](#license)

## Features

- **Efficient Encoding & Decoding**: Convert between standard UTF-8 strings and the compact DEC SIXBIT format.
- **Safety and Performance**: Offers both safe and unsafe encoding/decoding functions for flexibility.
- **Struct API**: Provides a `DecSixbit` struct for a more encapsulated and feature-rich API (enabled via the default `with-struct` feature).
- **Comprehensive Testing**: Includes a suite of tests to ensure reliability and correctness.
- **Error Handling**: Clearly defined error types for invalid input data.

## Installation

Add `dec-sixbit` to your project's `Cargo.toml`:

```toml
[dependencies]
dec-sixbit = "0.1.2" # Replace with the latest version
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
- `IInvalidBytesLength`: Occurs when decoding encounters iconsistent byte length and string length.

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
