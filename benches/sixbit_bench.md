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
| decode | short | 20.05 | ±0.93 |
| decode | medium | 39.54 | ±1.38 |
| decode | long | 65.94 | ±2.66 |
| decode_unchecked | short | 19.69 | ±0.86 |
| decode_unchecked | medium | 38.97 | ±2.78 |
| decode_unchecked | long | 65.52 | ±3.03 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 14.07 | ±1.08 |
| encode | medium | 16.43 | ±1.36 |
| encode | long | 46.69 | ±1.67 |
| encode_unchecked | short | 13.53 | ±0.50 |
| encode_unchecked | medium | 15.32 | ±0.41 |
| encode_unchecked | long | 15.03 | ±0.21 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 12.68 | ±0.40 |
| string_clone | medium | 14.39 | ±0.18 |
| string_clone | long | 14.37 | ±0.76 |
| string_from_utf8 | short | 20.62 | ±0.29 |
| string_from_utf8 | medium | 27.54 | ±1.89 |
| string_from_utf8 | long | 23.57 | ±2.65 |

## Key Observations

- Encode operations perform nearly as fast as baseline String clone operations
- Decode operations show minimal difference between checked and unchecked variants (less than 2% difference)
- Decode operations continue to scale linearly with input size
