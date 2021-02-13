use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Range,
    ptr, str,
};
use span::TextSpan;

use crate::span;

/// A cursor position in a `&str` which provides useful methods to manually parse that string.
#[derive(Clone, Copy)]
pub struct Position<'i> {
    input: &'i str,
    /// # Safety:
    ///
    /// `input[pos..]` must be a valid codepoint boundary (should not panic when indexing thus).
    position: usize,
}

impl<'i> Position<'i> {
    /// Create a new `Position` without checking invariants. (Checked with `debug_assertions`.)
    ///
    /// # Safety:
    ///
    /// `input[pos..]` must be a valid codepoint boundary (should not panic when indexing thus).
    pub(crate) unsafe fn new_unchecked(input: &str, pos: usize) -> Position<'_> {
        debug_assert!(input.get(pos..).is_some());
        Position { input, position: pos }
    }

    /// Attempts to create a new `Position` at the given position. If the specified position is
    /// an invalid index, or the specified position is not a valid UTF8 boundary, then None is
    /// returned.
    ///
    /// # Examples
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let cheart = '💖';
    /// let heart = "💖";
    /// assert_eq!(Position::new(heart, 1), None);
    /// assert_ne!(Position::new(heart, cheart.len_utf8()), None);
    /// ```
    pub fn new(input: &str, pos: usize) -> Option<Position<'_>> {
        input.get(pos..).map(|_| Position { input, position: pos })
    }

    /// Creates a `Position` at the start of a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let start = Position::from_start("");
    /// assert_eq!(start.offset(), 0);
    /// ```
    #[inline]
    pub fn from_start(input: &'i str) -> Position<'i> {
        // Position 0 is always safe because it's always a valid UTF-8 border.
        Position { input, position: 0 }
    }

    /// Returns the byte position of this `Position` as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let mut start = Position::from_start(input);
    ///
    /// assert_eq!(start.offset(), 0);
    /// ```
    #[inline]
    pub fn offset(&self) -> usize {
        self.position
    }

    /// Creates a `Span` from two `Position`s.
    ///
    /// # Panics
    ///
    /// Panics if the positions come from different inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let span = start.span(&start.clone());
    ///
    /// assert_eq!(span.start(), 0);
    /// assert_eq!(span.end(), 0);
    /// ```
    #[inline]
    pub fn span(&self, other: &Position<'i>) -> TextSpan<'i> {
        if ptr::eq(self.input, other.input) {
            // This is safe because the pos field of a Position should always be a valid str index.
            unsafe { TextSpan::new_unchecked(self.input, self.position, other.position) }
        }
        else {
            // TODO: maybe a panic if self.pos < other.pos
            panic!("span created from positions from different inputs")
        }
    }

    /// Returns the line and column number of this `Position`.
    ///
    /// This is an O(n) operation, where n is the number of chars in the input.
    /// You better use [`pair.line_col()`](struct.Pair.html#method.line_col) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "\na";
    /// let mut state: Box<yggdrasil_rt::State<'_, Rule>> = yggdrasil_rt::State::new(input);
    /// let mut result = state.match_string("\na");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().line_col(), (2, 2));
    /// ```
    #[inline]
    pub fn line_column(&self) -> (usize, usize) {
        if self.position > self.input.len() {
            panic!("position out of bounds");
        }
        let mut pos = self.position;
        let slice = &self.input[..pos];
        let mut chars = slice.chars().peekable();

        let mut line_col = (1, 1);

        while pos != 0 {
            match chars.next() {
                Some('\r') => {
                    if let Some(&'\n') = chars.peek() {
                        chars.next();

                        if pos == 1 {
                            pos -= 1;
                        }
                        else {
                            pos -= 2;
                        }

                        line_col = (line_col.0 + 1, 1);
                    }
                    else {
                        pos -= 1;
                        line_col = (line_col.0, line_col.1 + 1);
                    }
                }
                Some('\n') => {
                    pos -= 1;
                    line_col = (line_col.0 + 1, 1);
                }
                Some(c) => {
                    pos -= c.len_utf8();
                    line_col = (line_col.0, line_col.1 + 1);
                }
                None => unreachable!(),
            }
        }

        line_col
    }

    /// Returns the entire line of the input that contains this `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "\na";
    /// let mut state: Box<yggdrasil_rt::State<'_, Rule>> = yggdrasil_rt::State::new(input);
    /// let mut result = state.match_string("\na");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().line_of(), "a");
    /// ```
    #[inline]
    pub fn line_of(&self) -> &'i str {
        if self.position > self.input.len() {
            panic!("position out of bounds");
        };
        // Safe since start and end can only be valid UTF-8 borders.
        &self.input[self.find_line_start()..self.find_line_end()]
    }

    pub(crate) fn find_line_start(&self) -> usize {
        if self.input.is_empty() {
            return 0;
        };
        // Position's pos is always a UTF-8 border.
        let start = self.input.char_indices().rev().skip_while(|&(i, _)| i >= self.position).find(|&(_, c)| c == '\n');
        match start {
            Some((i, _)) => i + 1,
            None => 0,
        }
    }

    pub(crate) fn find_line_end(&self) -> usize {
        if self.input.is_empty() {
            0
        }
        else if self.position == self.input.len() - 1 {
            self.input.len()
        }
        else {
            // Position's pos is always a UTF-8 border.
            let end = self.input.char_indices().skip_while(|&(i, _)| i < self.position).find(|&(_, c)| c == '\n');
            match end {
                Some((i, _)) => i + 1,
                None => self.input.len(),
            }
        }
    }

    /// Returns `true` when the `Position` points to the start of the input `&str`.
    #[inline]
    pub(crate) fn match_soi(&self) -> bool {
        self.position == 0
    }

    /// Returns `true` when the `Position` points to the end of the input `&str`.
    #[inline]
    pub(crate) fn match_eoi(&self) -> bool {
        self.position == self.input.len()
    }

    /// Match rest of line, never fails
    #[inline]
    pub(crate) fn match_rol(&mut self) -> bool {
        let mut offset = 0;
        for c in self.rest_text().chars() {
            match c {
                '\n' | '\r' => break,
                _ => offset += c.len_utf8(),
            }
        }
        self.position += offset;
        return true;
    }

    /// Skips `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    pub(crate) fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[self.position..].chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                }
                else {
                    return false;
                }
            }
            len
        };

        self.position += skipped;
        true
    }

    /// Goes back `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn skip_back(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[..self.position].chars().rev();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                }
                else {
                    return false;
                }
            }
            len
        };

        self.position -= skipped;
        true
    }

    /// Matches the char at the `Position` against a specified character and returns `true` if a match
    /// was made. If no match was made, returns `false`.
    /// `pos` will not be updated in either case.
    #[allow(unused)]
    #[inline]
    pub(crate) fn match_char(&self, c: char) -> bool {
        matches!(self.input[self.position..].chars().next(), Some(cc) if c == cc)
    }

    /// Matches the char at the `Position` against a filter function and returns `true` if a match
    /// was made. If no match was made, returns `false` and `pos` will not be updated.
    #[inline]
    pub(crate) fn match_char_by<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        if let Some(c) = self.input[self.position..].chars().next() {
            if f(c) {
                self.position += c.len_utf8();
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Matches `string` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_string(&mut self, string: &str) -> bool {
        let to = self.position + string.len();

        if Some(string.as_bytes()) == self.input.as_bytes().get(self.position..to) {
            self.position = to;
            true
        }
        else {
            false
        }
    }

    /// Case-insensitively matches `string` from the `Position` and returns `true` if a match was
    /// made or `false` otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_insensitive(&mut self, string: &str) -> bool {
        let matched = {
            let slice = &self.input[self.position..];
            if let Some(slice) = slice.get(0..string.len()) { slice.eq_ignore_ascii_case(string) } else { false }
        };

        if matched {
            self.position += string.len();
            true
        }
        else {
            false
        }
    }

    /// Matches `char` `range` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.input[self.position..].chars().next() {
            if range.start <= c && c <= range.end {
                self.position += c.len_utf8();
                return true;
            }
        }

        false
    }

    #[inline]
    pub(crate) fn match_regex(&mut self, regex: &crate::Regex) -> bool {
        match regex.find(self.rest_text()) {
            Some(s) => {
                self.position += s.end();
                true
            }
            None => false,
        }
    }

    fn rest_text(&self) -> &str {
        if cfg!(debug_assertions) { &self.input[self.position..] } else { unsafe { self.input.get_unchecked(self.position..) } }
    }
}

impl<'i> fmt::Debug for Position<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position").field("pos", &self.position).finish()
    }
}

impl<'i> PartialEq for Position<'i> {
    fn eq(&self, other: &Position<'i>) -> bool {
        ptr::eq(self.input, other.input) && self.position == other.position
    }
}

impl<'i> Eq for Position<'i> {}

impl<'i> PartialOrd for Position<'i> {
    fn partial_cmp(&self, other: &Position<'i>) -> Option<Ordering> {
        if ptr::eq(self.input, other.input) { self.position.partial_cmp(&other.position) } else { None }
    }
}

impl<'i> Ord for Position<'i> {
    fn cmp(&self, other: &Position<'i>) -> Ordering {
        self.partial_cmp(other).expect("cannot compare positions from different strs")
    }
}

impl<'i> Hash for Position<'i> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.input as *const str).hash(state);
        self.position.hash(state);
    }
}
