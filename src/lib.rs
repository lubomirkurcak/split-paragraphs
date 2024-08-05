#![no_std]

use core::slice;
use core::str::{from_utf8_unchecked, Lines};

/// Trait extending [`str`] with the [`paragraphs`] method.
///
/// See its documentation for more.
///
/// [`paragraphs`]: ParagraphsExt::paragraphs
pub trait ParagraphsExt {
    /// An iterator over the paragraphs of a string, as string slices.
    ///
    /// Paragraphs consist of one or more lines of text containing non-whitespace
    /// characters surrounded by lines that only contain whitespace characters.
    ///
    /// [`paragraphs`] mirrors the behavior of the standard library's [`lines`].
    /// See its documentation for more details.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use split_paragraphs::ParagraphsExt;
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
    /// # use split_paragraphs::ParagraphsExt;
    /// let text = "\n\n\nfoo\nbar\n\r\nbaz";
    /// let mut paragraphs = text.paragraphs();
    ///
    /// assert_eq!(Some("foo\nbar"), paragraphs.next());
    /// assert_eq!(Some("baz"), paragraphs.next());
    ///
    /// assert_eq!(None, paragraphs.next());
    /// ```
    fn paragraphs(&self) -> Paragraphs;
}

impl ParagraphsExt for str {
    fn paragraphs(&self) -> Paragraphs {
        Paragraphs {
            lines: self.lines(),
        }
    }
}

/// An iterator over the paragraphs of a string, as string slices.
///
/// This struct is created with the [`paragraphs`] method on [`str`] via
/// the [`ParagraphsExt`] trait.
/// See its documentation for more.
///
/// [`paragraphs`]: ParagraphsExt::paragraphs
pub struct Paragraphs<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for Paragraphs<'a> {
    type Item = &'a str;

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
            let line = self.lines.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();
            if line.trim().is_empty() {
                break;
            }
            last_non_empty_line = line;
        }

        let result: &str = unsafe {
            from_utf8_unchecked(slice::from_raw_parts(
                first_non_empty_line.as_ptr(),
                last_non_empty_line
                    .as_ptr()
                    .offset_from(first_non_empty_line.as_ptr()) as usize
                    + last_non_empty_line.len(),
            ))
        };

        Some(result)
    }
}

impl<'a> DoubleEndedIterator for Paragraphs<'a> {
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
            let line = self.lines.next_back();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();
            if line.trim().is_empty() {
                break;
            }
            first_non_empty_line = line;
        }

        let result: &str = unsafe {
            from_utf8_unchecked(slice::from_raw_parts(
                first_non_empty_line.as_ptr(),
                last_non_empty_line
                    .as_ptr()
                    .offset_from(first_non_empty_line.as_ptr()) as usize
                    + last_non_empty_line.len(),
            ))
        };

        Some(result)
    }
}
