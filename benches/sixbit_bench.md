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
| decode | short | 25.99 | ±0.81 |
| decode | medium | 46.50 | ±2.55 |
| decode | long | 74.11 | ±2.82 |
| decode_unchecked | short | 20.87 | ±1.15 |
| decode_unchecked | medium | 40.76 | ±2.60 |
| decode_unchecked | long | 67.04 | ±2.36 |

## Sixbit Encode Operations

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| encode | short | 20.34 | ±0.50 |
| encode | medium | 35.93 | ±2.00 |
| encode | long | 56.37 | ±1.09 |
| encode_unchecked | short | 18.85 | ±0.49 |
| encode_unchecked | medium | 30.77 | ±0.72 |
| encode_unchecked | long | 44.71 | ±2.38 |

## String Operations (Baseline)

| Operation | Input Size | Time (ns/iter) | Margin of Error |
|-----------|------------|----------------|-----------------|
| string_clone | short | 12.64 | ±0.16 |
| string_clone | medium | 14.66 | ±0.46 |
| string_clone | long | 14.66 | ±0.47 |
| string_from_utf8 | short | 20.99 | ±0.49 |
| string_from_utf8 | medium | 26.98 | ±0.80 |
| string_from_utf8 | long | 23.43 | ±1.70 |

## Key Observations

- Unchecked operations are consistently faster than their checked counterparts
- Operation time scales with input size as expected
