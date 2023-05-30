# Dart Semver
[![Crates.io](https://img.shields.io/crates/v/dart-semver?color=blue)](https://crates.io/crates/dart-semver) [![docs.rs](https://img.shields.io/docsrs/dart-semver?label=docs.rs&logo=docs.rs)](https://docs.rs/dart-semver/latest/dart_semver/) [![ci](https://github.com/Yakiyo/dart-semver/actions/workflows/ci.yml/badge.svg)](https://github.com/Yakiyo/dart-semver)

Rust crate for parsing dart sdk's semver

Pure rust implementation to parse a dart sdk version string to a rust struct.

Reference: https://github.com/dart-lang/sdk/blob/main/tools/VERSION

## Usage
Add the crate to Cargo.toml
```bash
$ cargo add dart-semver
```

```rs
use dart_semver::{Version, Channel};

let v = Version::parse("4.3.4").unwrap();

assert_eq!(v, Version {
    major: 4,
    minor: 3,
    patch: 4,
    prerelease: None,
    prerelease_patch: None,
    channel: Channel::Stable
})

```
