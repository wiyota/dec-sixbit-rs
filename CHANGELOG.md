# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.1.2] - 2025-01-27

### Added

- Added new string manipulation methods to `DecSixbit` struct.
- Added unit tests for `DecSixbit` struct and decoding functions.

### Changed

- Made `*_unchecked()` functions safe (no longer marked as `unsafe`).

### Fixed

- Clarified error message for invalid character in input during encoding.
- Updated error handling for inconsistent input bytes and length.

## [0.1.1] - 2025-01-26

- Added some derived traits to the `Error` type.
- Added a check before encoding if the input is an ASCII string.

## [0.1.0] - 2025-01-26

- Initial release.
