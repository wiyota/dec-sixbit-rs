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
| decode | short | 26.58 | ±1.60 |
| decode | medium | 47.00 | ±4.65 |
| decode | long | 75.31 | ±3.54 |
| decode_unchecked | short | 21.16 | ±0.72 |
| decode_unchecked | medium | 40.84 | ±1.49 |
| decode_unchecked | long | 68.22 | ±4.69 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 21.86 | ±0.60 |
| encode | medium | 39.58 | ±1.37 |
| encode | long | 64.24 | ±14.50 |
| encode_unchecked | short | 19.10 | ±0.87 |
| encode_unchecked | medium | 31.01 | ±1.22 |
| encode_unchecked | long | 45.47 | ±1.01 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 13.04 | ±0.50 |
| string_clone | medium | 14.55 | ±1.46 |
| string_clone | long | 14.57 | ±2.09 |
| string_from_utf8 | short | 21.11 | ±1.59 |
| string_from_utf8 | medium | 27.27 | ±0.68 |
| string_from_utf8 | long | 23.77 | ±2.08 |

## Key Observations

- Unchecked operations are consistently faster than their checked counterparts
- Operation time scales with input size as expected
