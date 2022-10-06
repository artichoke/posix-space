# posix-space

[![GitHub Actions](https://github.com/artichoke/posix-space/workflows/CI/badge.svg)](https://github.com/artichoke/posix-space/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/posix-space.svg)](https://crates.io/crates/posix-space)
[![API](https://docs.rs/posix-space/badge.svg)](https://docs.rs/posix-space)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/posix-space/posix_space/)

A small crate which determines if a byte is classified as a space in the POSIX
locale per [POSIX.1-2017], chapter 7, [Locale].

[posix.1-2017]: https://pubs.opengroup.org/onlinepubs/9699919799/mindex.html
[locale]:
  https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap07.html

> **space**
>
> Define characters to be classified as white-space characters.
>
> In the POSIX locale, exactly \<space\>, \<form-feed\>, \<newline\>,
> \<carriage-return\>, \<tab\>, and \<vertical-tab\> shall be included.

The function defined in this crate should have equivalent behavior to the C
function [`isspace`] as defined in `ctype.h`.

[`isspace`]: https://linux.die.net/man/3/isspace

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
posix-space = "1.0.2"
```

Then classify bytes like:

```rust
assert!(posix_space::is_space(b' '));
assert!(posix_space::is_space(b'\t'));
assert!(posix_space::is_space(b'\r'));

assert!(!posix_space::is_space(b'\0'));
assert!(!posix_space::is_space(b'C'));
assert!(!posix_space::is_space(b'&'));
```

This crate's behavior differs from [`u8::is_ascii_whitespace`] in the Rust
standard library in that \<vertical-tab\>, `\x0B`, is considered a **space**.

[`u8::is_ascii_whitespace`]:
  https://doc.rust-lang.org/stable/std/primitive.u8.html#method.is_ascii_whitespace

```rust
assert!(posix_space::is_space(b'\x0B'));
```

## Crate features

`posix-space` is `no_std` with no dependencies outside of Rust [`core`].

### Minimum Supported Rust Version

This crate requires at least Rust 1.31.0. This version can be bumped in minor
releases.

## License

`posix-space` is licensed under the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`core`]: https://doc.rust-lang.org/stable/core/
