#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! A small crate which determines if a byte is classified as a space in the
//! POSIX locale per [POSIX.1-2017], chapter 7, [Locale].
//!
//! [POSIX.1-2017]: https://pubs.opengroup.org/onlinepubs/9699919799/mindex.html
//! [Locale]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap07.html
//!
//! > **space**
//! >
//! > Define characters to be classified as white-space characters.
//! >
//! > In the POSIX locale, exactly \<space\>, \<form-feed\>, \<newline\>, \<carriage-return\>,
//! > \<tab\>, and \<vertical-tab\> shall be included.
//!
//! The function defined in this crate should have equivalent behavior to the C
//! fucntion [`isspace`] as defined in `ctype.h`.
//!
//! [`isspace`]: https://linux.die.net/man/3/isspace

#![doc(html_root_url = "https://docs.rs/posix-space/1.0.2")]
#![no_std]

/// Determine whether the given byte is in **space** POSIX character class.
///
/// In the POSIX locale, exactly \<space\>, \<form-feed\>, \<newline\>,
/// \<carriage-return\>, \<tab\>, and \<vertical-tab\> shall be included.
///
/// # Compatibility
///
/// This function differs from [`u8::is_ascii_whitespace`] in that \<vertical-tab\>,
/// `\x0B`, is considered a **space**.
///
/// # Examples
///
/// ```
/// assert!(posix_space::is_space(b' '));
/// assert!(posix_space::is_space(b'\x0C'));
/// assert!(posix_space::is_space(b'\n'));
/// assert!(posix_space::is_space(b'\r'));
/// assert!(posix_space::is_space(b'\t'));
/// assert!(posix_space::is_space(b'\x0B'));
/// ```
///
/// Other ASCII characters are not POSIX spaces:
///
/// ```
/// assert!(!posix_space::is_space(b'C'));
/// assert!(!posix_space::is_space(b'&'));
/// assert!(!posix_space::is_space(b'\x7F'));
/// ```
///
/// Non-ASCII bytes are not POSIX spaces:
///
/// ```
/// assert!(!posix_space::is_space(b'\x80'));
/// assert!(!posix_space::is_space(b'\xFF'));
/// ```
#[must_use]
pub fn is_space(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte == b'\x0B'
}

#[cfg(test)]
mod tests {
    use super::*;

    // ```
    // [3.1.2] > (0..255).each { |b| puts "(0x#{b.to_s(16).upcase}, #{b.chr.match?(/[[:space:]]/)}, #{b.chr.inspect})," }
    // ```
    const BYTE_TO_POSIX_SPACE: [(u8, bool, &str); 256] = [
        (0x0, false, r"\x00"),
        (0x1, false, r"\x01"),
        (0x2, false, r"\x02"),
        (0x3, false, r"\x03"),
        (0x4, false, r"\x04"),
        (0x5, false, r"\x05"),
        (0x6, false, r"\x06"),
        (0x7, false, r"\a"),
        (0x8, false, r"\b"),
        (0x9, true, r"\t"),
        (0xA, true, r"\n"),
        (0xB, true, r"\v"),
        (0xC, true, r"\f"),
        (0xD, true, r"\r"),
        (0xE, false, r"\x0E"),
        (0xF, false, r"\x0F"),
        (0x10, false, r"\x10"),
        (0x11, false, r"\x11"),
        (0x12, false, r"\x12"),
        (0x13, false, r"\x13"),
        (0x14, false, r"\x14"),
        (0x15, false, r"\x15"),
        (0x16, false, r"\x16"),
        (0x17, false, r"\x17"),
        (0x18, false, r"\x18"),
        (0x19, false, r"\x19"),
        (0x1A, false, r"\x1A"),
        (0x1B, false, r"\e"),
        (0x1C, false, r"\x1C"),
        (0x1D, false, r"\x1D"),
        (0x1E, false, r"\x1E"),
        (0x1F, false, r"\x1F"),
        (0x20, true, " "),
        (0x21, false, "!"),
        (0x22, false, r#"\""#),
        (0x23, false, "#"),
        (0x24, false, "$"),
        (0x25, false, "%"),
        (0x26, false, "&"),
        (0x27, false, "'"),
        (0x28, false, "("),
        (0x29, false, ")"),
        (0x2A, false, "*"),
        (0x2B, false, "+"),
        (0x2C, false, ","),
        (0x2D, false, "-"),
        (0x2E, false, "."),
        (0x2F, false, "/"),
        (0x30, false, "0"),
        (0x31, false, "1"),
        (0x32, false, "2"),
        (0x33, false, "3"),
        (0x34, false, "4"),
        (0x35, false, "5"),
        (0x36, false, "6"),
        (0x37, false, "7"),
        (0x38, false, "8"),
        (0x39, false, "9"),
        (0x3A, false, ":"),
        (0x3B, false, ";"),
        (0x3C, false, "<"),
        (0x3D, false, "="),
        (0x3E, false, ">"),
        (0x3F, false, "?"),
        (0x40, false, "@"),
        (0x41, false, "A"),
        (0x42, false, "B"),
        (0x43, false, "C"),
        (0x44, false, "D"),
        (0x45, false, "E"),
        (0x46, false, "F"),
        (0x47, false, "G"),
        (0x48, false, "H"),
        (0x49, false, "I"),
        (0x4A, false, "J"),
        (0x4B, false, "K"),
        (0x4C, false, "L"),
        (0x4D, false, "M"),
        (0x4E, false, "N"),
        (0x4F, false, "O"),
        (0x50, false, "P"),
        (0x51, false, "Q"),
        (0x52, false, "R"),
        (0x53, false, "S"),
        (0x54, false, "T"),
        (0x55, false, "U"),
        (0x56, false, "V"),
        (0x57, false, "W"),
        (0x58, false, "X"),
        (0x59, false, "Y"),
        (0x5A, false, "Z"),
        (0x5B, false, "["),
        (0x5C, false, r"\\"),
        (0x5D, false, "]"),
        (0x5E, false, "^"),
        (0x5F, false, "_"),
        (0x60, false, "`"),
        (0x61, false, "a"),
        (0x62, false, "b"),
        (0x63, false, "c"),
        (0x64, false, "d"),
        (0x65, false, "e"),
        (0x66, false, "f"),
        (0x67, false, "g"),
        (0x68, false, "h"),
        (0x69, false, "i"),
        (0x6A, false, "j"),
        (0x6B, false, "k"),
        (0x6C, false, "l"),
        (0x6D, false, "m"),
        (0x6E, false, "n"),
        (0x6F, false, "o"),
        (0x70, false, "p"),
        (0x71, false, "q"),
        (0x72, false, "r"),
        (0x73, false, "s"),
        (0x74, false, "t"),
        (0x75, false, "u"),
        (0x76, false, "v"),
        (0x77, false, "w"),
        (0x78, false, "x"),
        (0x79, false, "y"),
        (0x7A, false, "z"),
        (0x7B, false, "{"),
        (0x7C, false, "|"),
        (0x7D, false, "}"),
        (0x7E, false, "~"),
        (0x7F, false, r"\x7F"),
        (0x80, false, r"\x80"),
        (0x81, false, r"\x81"),
        (0x82, false, r"\x82"),
        (0x83, false, r"\x83"),
        (0x84, false, r"\x84"),
        (0x85, false, r"\x85"),
        (0x86, false, r"\x86"),
        (0x87, false, r"\x87"),
        (0x88, false, r"\x88"),
        (0x89, false, r"\x89"),
        (0x8A, false, r"\x8A"),
        (0x8B, false, r"\x8B"),
        (0x8C, false, r"\x8C"),
        (0x8D, false, r"\x8D"),
        (0x8E, false, r"\x8E"),
        (0x8F, false, r"\x8F"),
        (0x90, false, r"\x90"),
        (0x91, false, r"\x91"),
        (0x92, false, r"\x92"),
        (0x93, false, r"\x93"),
        (0x94, false, r"\x94"),
        (0x95, false, r"\x95"),
        (0x96, false, r"\x96"),
        (0x97, false, r"\x97"),
        (0x98, false, r"\x98"),
        (0x99, false, r"\x99"),
        (0x9A, false, r"\x9A"),
        (0x9B, false, r"\x9B"),
        (0x9C, false, r"\x9C"),
        (0x9D, false, r"\x9D"),
        (0x9E, false, r"\x9E"),
        (0x9F, false, r"\x9F"),
        (0xA0, false, r"\xA0"),
        (0xA1, false, r"\xA1"),
        (0xA2, false, r"\xA2"),
        (0xA3, false, r"\xA3"),
        (0xA4, false, r"\xA4"),
        (0xA5, false, r"\xA5"),
        (0xA6, false, r"\xA6"),
        (0xA7, false, r"\xA7"),
        (0xA8, false, r"\xA8"),
        (0xA9, false, r"\xA9"),
        (0xAA, false, r"\xAA"),
        (0xAB, false, r"\xAB"),
        (0xAC, false, r"\xAC"),
        (0xAD, false, r"\xAD"),
        (0xAE, false, r"\xAE"),
        (0xAF, false, r"\xAF"),
        (0xB0, false, r"\xB0"),
        (0xB1, false, r"\xB1"),
        (0xB2, false, r"\xB2"),
        (0xB3, false, r"\xB3"),
        (0xB4, false, r"\xB4"),
        (0xB5, false, r"\xB5"),
        (0xB6, false, r"\xB6"),
        (0xB7, false, r"\xB7"),
        (0xB8, false, r"\xB8"),
        (0xB9, false, r"\xB9"),
        (0xBA, false, r"\xBA"),
        (0xBB, false, r"\xBB"),
        (0xBC, false, r"\xBC"),
        (0xBD, false, r"\xBD"),
        (0xBE, false, r"\xBE"),
        (0xBF, false, r"\xBF"),
        (0xC0, false, r"\xC0"),
        (0xC1, false, r"\xC1"),
        (0xC2, false, r"\xC2"),
        (0xC3, false, r"\xC3"),
        (0xC4, false, r"\xC4"),
        (0xC5, false, r"\xC5"),
        (0xC6, false, r"\xC6"),
        (0xC7, false, r"\xC7"),
        (0xC8, false, r"\xC8"),
        (0xC9, false, r"\xC9"),
        (0xCA, false, r"\xCA"),
        (0xCB, false, r"\xCB"),
        (0xCC, false, r"\xCC"),
        (0xCD, false, r"\xCD"),
        (0xCE, false, r"\xCE"),
        (0xCF, false, r"\xCF"),
        (0xD0, false, r"\xD0"),
        (0xD1, false, r"\xD1"),
        (0xD2, false, r"\xD2"),
        (0xD3, false, r"\xD3"),
        (0xD4, false, r"\xD4"),
        (0xD5, false, r"\xD5"),
        (0xD6, false, r"\xD6"),
        (0xD7, false, r"\xD7"),
        (0xD8, false, r"\xD8"),
        (0xD9, false, r"\xD9"),
        (0xDA, false, r"\xDA"),
        (0xDB, false, r"\xDB"),
        (0xDC, false, r"\xDC"),
        (0xDD, false, r"\xDD"),
        (0xDE, false, r"\xDE"),
        (0xDF, false, r"\xDF"),
        (0xE0, false, r"\xE0"),
        (0xE1, false, r"\xE1"),
        (0xE2, false, r"\xE2"),
        (0xE3, false, r"\xE3"),
        (0xE4, false, r"\xE4"),
        (0xE5, false, r"\xE5"),
        (0xE6, false, r"\xE6"),
        (0xE7, false, r"\xE7"),
        (0xE8, false, r"\xE8"),
        (0xE9, false, r"\xE9"),
        (0xEA, false, r"\xEA"),
        (0xEB, false, r"\xEB"),
        (0xEC, false, r"\xEC"),
        (0xED, false, r"\xED"),
        (0xEE, false, r"\xEE"),
        (0xEF, false, r"\xEF"),
        (0xF0, false, r"\xF0"),
        (0xF1, false, r"\xF1"),
        (0xF2, false, r"\xF2"),
        (0xF3, false, r"\xF3"),
        (0xF4, false, r"\xF4"),
        (0xF5, false, r"\xF5"),
        (0xF6, false, r"\xF6"),
        (0xF7, false, r"\xF7"),
        (0xF8, false, r"\xF8"),
        (0xF9, false, r"\xF9"),
        (0xFA, false, r"\xFA"),
        (0xFB, false, r"\xFB"),
        (0xFC, false, r"\xFC"),
        (0xFD, false, r"\xFD"),
        (0xFE, false, r"\xFE"),
        (0xFF, false, r"\xFF"),
    ];

    #[test]
    fn space_character_class() {
        let test_cases = BYTE_TO_POSIX_SPACE;
        for &(byte, is_posix_space, display) in test_cases.iter() {
            assert_eq!(
                is_space(byte),
                is_posix_space,
                "Mismatch for {} - {}",
                byte,
                display
            );
        }
    }

    #[test]
    fn non_ascii_bytes_are_not_posix_spaces() {
        for byte in 0..core::u8::MAX {
            if byte.is_ascii() {
                continue;
            }
            assert!(
                !is_space(byte),
                "non-ascii byte {} was classified as a POSIX space",
                byte
            );
        }
    }
}

// Ensure code blocks in README.md compile
//
// This module and macro declaration should be kept at the end of the file, in
// order to not interfere with code coverage.
#[cfg(doctest)]
macro_rules! readme {
    ($x:expr) => {
        #[doc = $x]
        mod readme {}
    };
    () => {
        readme!(include_str!("../README.md"));
    };
}
#[cfg(doctest)]
readme!();
