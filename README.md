# cvt
[![Build Status](https://travis-ci.org/marmistrz/cvt.svg?branch=master)](https://travis-ci.org/marmistrz/cvt)
[![Docs](https://docs.rs/cvt/badge.svg)](https://docs.rs/cvt)
[![crates-io-badge]][crates-io]

[crates-io-badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true
[crates-io]: https://crates.io/crates/cvt

This package exposes the `cvt` function used extensively by `libstd` to
convert platform-specific syscall error codes to `std::io::Result`.

Usually syscalls use return values for errors, the conventions differ. For instance,
on Unix `0` means success on Unix but failure on Windows.

While those conventions are not always followed, they usually are and
`cvt` is there to reduce the mental bookkeeping and make it easier to handle syscall errors.

The code was mostly copied over from Rust libstd, because the function is not public.
