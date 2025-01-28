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
| decode | short | 17.57 | ±1.12 |
| decode | medium | 22.76 | ±0.64 |
| decode | long | 27.73 | ±1.29 |
| decode_unchecked | short | 16.74 | ±0.76 |
| decode_unchecked | medium | 21.99 | ±0.81 |
| decode_unchecked | long | 27.75 | ±0.69 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 14.06 | ±0.42 |
| encode | medium | 16.18 | ±0.39 |
| encode | long | 47.20 | ±2.96 |
| encode_unchecked | short | 13.75 | ±0.68 |
| encode_unchecked | medium | 15.52 | ±0.55 |
| encode_unchecked | long | 15.24 | ±0.40 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 12.64 | ±0.06 |
| string_clone | medium | 14.54 | ±0.42 |
| string_clone | long | 14.65 | ±0.55 |
| string_from_utf8 | short | 20.89 | ±1.52 |
| string_from_utf8 | medium | 26.96 | ±1.09 |
| string_from_utf8 | long | 23.03 | ±0.74 |

## Key Observations

- Encode operations maintain superior performance for unchecked operations on long inputs (3x faster than checked)
- Encode and decode operations on short inputs remain competitive with baseline String operations
- Both checked and unchecked variants show minimal performance difference in decode operations
