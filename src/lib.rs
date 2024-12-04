//! A crate that provides paragraph iteration for strings.
//!
//! This crate extends [`str`] with the ability to iterate over paragraphs via the [`SplitParagraphs`] trait.
//! A paragraph is defined as one or more consecutive non-empty lines, separated by one or more blank lines.
//!
//! # Example
//! ```
//! use split_paragraphs::SplitParagraphs;
//!
//! let text = "foo\r\nbar\n\nbaz\r";
//! let mut paragraphs = text.paragraphs();
//!
//! assert_eq!(paragraphs.next(), Some("foo\r\nbar"));
//! assert_eq!(paragraphs.next(), Some("baz\r"));
//! assert_eq!(paragraphs.next(), None);
//! ```

#![no_std]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::cast_sign_loss)]

use core::iter::FusedIterator;
use core::slice;
use core::str::{from_utf8_unchecked, Lines};

/// Trait extending [`str`] with [`paragraphs`].
///
/// [`paragraphs`]: SplitParagraphs::paragraphs
pub trait SplitParagraphs {
    /// Returns an iterator over paragraphs of a string, as string slices.
    ///
    /// A paragraph consists of one or more lines containing non-whitespace characters,
    /// separated by empty lines or lines containing only whitespace.
    ///
    /// Paragraphs always contain at least one line with at least one non-whitespace
    /// character.
    ///
    /// Paragraphs never contain empty lines or whitespace-only lines.
    ///
    /// Paragraphs support line endings that are either newlines (`\n`) or
    /// carriage return followed by line feed (`\r\n`).
    ///
    /// Line terminators between paragraphs are not included in the returned slices.
    ///
    /// Line terminators within paragraphs are preserved in their original form.
    ///
    /// Handling of line endings matches [`lines`]. See its documentation for more details.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use split_paragraphs::SplitParagraphs;
    /// let text = "foo\r\nbar\n\nbaz\r";
    /// let mut paragraphs = text.paragraphs();
    ///
    /// assert_eq!(Some("foo\r\nbar"), paragraphs.next());
    /// // Trailing carriage return is included in the last paragraph
    /// assert_eq!(Some("baz\r"), paragraphs.next());
    ///
    /// assert_eq!(None, paragraphs.next());
    /// ```
    ///
    /// The final paragraph does not require any ending:
    ///
    /// ```
    /// # use split_paragraphs::SplitParagraphs;
    /// let text = "\n\n\nfoo\nbar\n\r\nbaz";
    /// let mut paragraphs = text.paragraphs();
    ///
    /// assert_eq!(Some("foo\nbar"), paragraphs.next());
    /// assert_eq!(Some("baz"), paragraphs.next());
    ///
    /// assert_eq!(None, paragraphs.next());
    /// ```
    ///
    /// [`paragraphs`]: SplitParagraphs::paragraphs
    /// [`lines`]: str::lines
    fn paragraphs(&self) -> Paragraphs;
}

/// An iterator over the paragraphs of a string, as string slices.
///
/// This struct is created with the [`paragraphs`] method on [`str`] via
/// the [`SplitParagraphs`] trait.
/// See its documentation for more.
///
/// [`paragraphs`]: SplitParagraphs::paragraphs
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Paragraphs<'a> {
    lines: Lines<'a>,
}

impl SplitParagraphs for str {
    #[inline]
    fn paragraphs(&self) -> Paragraphs {
        Paragraphs {
            lines: self.lines(),
        }
    }
}

impl<'a> Iterator for Paragraphs<'a> {
    type Item = &'a str;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.lines.size_hint().1.map(|n| (n + 1) / 2))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let first_line = self.lines.next()?;

        let first_non_empty_line = if first_line.trim().is_empty() {
            loop {
                let line = self.lines.next()?;
                if !line.trim().is_empty() {
                    break line;
                }
            }
        } else {
            first_line
        };

        let mut last_non_empty_line = first_non_empty_line;
        loop {
            let Some(line) = self.lines.next() else {
                break;
            };
            if line.trim().is_empty() {
                break;
            }
            last_non_empty_line = line;
        }

        let result: &str = unsafe {
            from_utf8_unchecked(slice::from_raw_parts(
                first_non_empty_line.as_ptr(),
                (last_non_empty_line
                    .as_ptr()
                    .offset_from(first_non_empty_line.as_ptr()) as usize)
                    .unchecked_add(last_non_empty_line.len()),
            ))
        };

        Some(result)
    }
}

impl DoubleEndedIterator for Paragraphs<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let last_line = self.lines.next_back()?;

        let last_non_empty_line = if last_line.trim().is_empty() {
            loop {
                let line = self.lines.next_back()?;
                if !line.trim().is_empty() {
                    break line;
                }
            }
        } else {
            last_line
        };

        let mut first_non_empty_line = last_non_empty_line;
        loop {
            let Some(line) = self.lines.next_back() else {
                break;
            };
            if line.trim().is_empty() {
                break;
            }
            first_non_empty_line = line;
        }

        let result: &str = unsafe {
            from_utf8_unchecked(slice::from_raw_parts(
                first_non_empty_line.as_ptr(),
                (last_non_empty_line
                    .as_ptr()
                    .offset_from(first_non_empty_line.as_ptr()) as usize)
                    .unchecked_add(last_non_empty_line.len()),
            ))
        };

        Some(result)
    }
}

impl FusedIterator for Paragraphs<'_> {}
