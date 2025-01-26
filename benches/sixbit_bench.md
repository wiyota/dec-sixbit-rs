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
| decode | short | 21.50 | ±0.98 |
| decode | medium | 39.85 | ±1.46 |
| decode | long | 66.87 | ±2.10 |
| decode_unchecked | short | 21.46 | ±1.25 |
| decode_unchecked | medium | 41.05 | ±1.29 |
| decode_unchecked | long | 66.99 | ±5.52 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 21.02 | ±0.81 |
| encode | medium | 38.27 | ±1.88 |
| encode | long | 60.49 | ±1.86 |
| encode_unchecked | short | 18.80 | ±1.15 |
| encode_unchecked | medium | 30.79 | ±0.85 |
| encode_unchecked | long | 45.07 | ±1.22 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 12.96 | ±4.33 |
| string_clone | medium | 14.83 | ±0.49 |
| string_clone | long | 14.78 | ±0.57 |
| string_from_utf8 | short | 21.13 | ±0.52 |
| string_from_utf8 | medium | 27.09 | ±0.76 |
| string_from_utf8 | long | 23.41 | ±1.06 |

## Key Observations

- Encode unchecked operations show significant performance improvements (11-25% faster) compared to checked counterparts
- Decode operations show minimal difference between checked and unchecked variants
- Both encode and decode operations maintain competitive performance compared to baseline String operations
- Operation time scales linearly with input size across all operations
