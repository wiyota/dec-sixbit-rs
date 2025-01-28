<!--
 Copyright 2025 Inomoto, Yota

 Licensed under the Apache License, Version 2.0 (the "License") or MIT License;
- Operating System: macOS Sequoia
- Architecture: Apple Silicon M2 Pro 16GB
-->

# Benchmark Results

## Sixbit Decode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| decode | short | 17.21 | ±0.95 |
| decode | medium | 27.66 | ±1.40 |
| decode | long | 40.37 | ±1.52 |
| decode_unchecked | short | 16.72 | ±0.76 |
| decode_unchecked | medium | 27.46 | ±1.21 |
| decode_unchecked | long | 40.26 | ±3.83 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 13.81 | ±0.07 |
| encode | medium | 16.15 | ±0.95 |
| encode | long | 47.25 | ±3.02 |
| encode_unchecked | short | 13.74 | ±1.00 |
| encode_unchecked | medium | 15.46 | ±1.10 |
| encode_unchecked | long | 14.97 | ±0.58 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 12.83 | ±0.27 |
| string_clone | medium | 14.68 | ±0.35 |
| string_clone | long | 14.88 | ±0.50 |
| string_from_utf8 | short | 20.95 | ±1.30 |
| string_from_utf8 | medium | 27.07 | ±1.11 |
| string_from_utf8 | long | 22.92 | ±1.71 |

## Key Observations

- Encode operations maintain their excellent performance, particularly for unchecked operations on long inputs (3x faster than checked)
- Decode and encode operations on short inputs show comparable performance to baseline String operations
- Both checked and unchecked variants perform similarly for decode operations (less than 2% difference)
