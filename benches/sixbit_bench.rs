// To run the benchmarks:
//
// ```sh
// # Switch to nightly Rust
// rustup override set nightly
//
// # Run benchmarks
// cargo bench --features nightly
// ```
//
// You can keep using stable Rust for normal development since all benchmark code is behind the nightly feature flag.

#![allow(dead_code)]
#![cfg_attr(feature = "nightly", feature(test))]

const SHORT_INPUT: &str = "HELLO WORLD";
const MEDIUM_INPUT: &str = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
const LONG_INPUT: &str = "SPHINX OF BLACK QUARTZ, JUDGE MY VOW! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG 1234567890";

#[cfg(feature = "nightly")]
mod benches {
    extern crate test;
    use super::*;
    use dec_sixbit::{decode, decode_unchecked, encode, encode_unchecked};
    use test::Bencher;

    #[bench]
    fn bench_string_clone_short(b: &mut Bencher) {
        b.iter(|| {
            SHORT_INPUT.to_string()
        });
    }

    #[bench]
    fn bench_sixbit_encode_short(b: &mut Bencher) {
        b.iter(|| {
            encode(SHORT_INPUT).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_encode_unchecked_short(b: &mut Bencher) {
        b.iter(|| {
            unsafe { encode_unchecked(SHORT_INPUT) }
        });
    }

    #[bench]
    fn bench_string_clone_medium(b: &mut Bencher) {
        b.iter(|| {
            MEDIUM_INPUT.to_string()
        });
    }

    #[bench]
    fn bench_sixbit_encode_medium(b: &mut Bencher) {
        b.iter(|| {
            encode(MEDIUM_INPUT).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_encode_unchecked_medium(b: &mut Bencher) {
        b.iter(|| {
            unsafe { encode_unchecked(MEDIUM_INPUT) }
        });
    }

    #[bench]
    fn bench_string_clone_long(b: &mut Bencher) {
        b.iter(|| {
            LONG_INPUT.to_string()
        });
    }

    #[bench]
    fn bench_sixbit_encode_long(b: &mut Bencher) {
        b.iter(|| {
            encode(LONG_INPUT).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_encode_unchecked_long(b: &mut Bencher) {
        b.iter(|| {
            unsafe { encode_unchecked(LONG_INPUT) }
        });
    }

    // Decode benchmarks with pre-encoded data
    #[bench]
    fn bench_string_from_utf8_short(b: &mut Bencher) {
        let input = SHORT_INPUT.as_bytes().to_vec();
        b.iter(|| {
            String::from_utf8(input.clone()).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_short(b: &mut Bencher) {
        let (input, len) = encode(SHORT_INPUT).unwrap();
        b.iter(|| {
            decode(&input, len).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_unchecked_short(b: &mut Bencher) {
        let (input, len) = unsafe { encode_unchecked(SHORT_INPUT) };
        b.iter(|| {
            unsafe { decode_unchecked(&input, len) }
        });
    }

    #[bench]
    fn bench_string_from_utf8_medium(b: &mut Bencher) {
        let input = MEDIUM_INPUT.as_bytes().to_vec();
        b.iter(|| {
            String::from_utf8(input.clone()).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_medium(b: &mut Bencher) {
        let (input, len) = encode(MEDIUM_INPUT).unwrap();
        b.iter(|| {
            decode(&input, len).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_unchecked_medium(b: &mut Bencher) {
        let (input, len) = unsafe { encode_unchecked(MEDIUM_INPUT) };
        b.iter(|| {
            unsafe { decode_unchecked(&input, len) }
        });
    }

    #[bench]
    fn bench_string_from_utf8_long(b: &mut Bencher) {
        let input = LONG_INPUT.as_bytes().to_vec();
        b.iter(|| {
            String::from_utf8(input.clone()).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_long(b: &mut Bencher) {
        let (input, len) = encode(LONG_INPUT).unwrap();
        b.iter(|| {
            decode(&input, len).unwrap()
        });
    }

    #[bench]
    fn bench_sixbit_decode_unchecked_long(b: &mut Bencher) {
        let (input, len) = unsafe { encode_unchecked(LONG_INPUT) };
        b.iter(|| {
            unsafe { decode_unchecked(&input, len) }
        });
    }
}
