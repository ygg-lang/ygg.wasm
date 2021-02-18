use super::*;

/// A pattern that matches a number with a leading zero and a base mark,
/// such as `0x0123456789ABCDEF`, `0o01234567`, `0b01010101`.
///
///
/// # Examples
///
/// ```
/// # use pex::{ParseState, helpers::ZeroBytePattern};
/// let lower = ParseState::new("0x1234");
/// let upper = ParseState::new("0X1234");
/// let bytes = ZeroBytePattern::new(&[("0x", 16), ("0o", 8), ("0b", 2)]);
/// assert!(bytes(lower).is_success());
/// assert!(bytes(upper).is_failure());
/// let bytes = bytes.with_insensitive(true);
/// assert!(bytes(lower).is_success());
/// assert!(bytes(upper).is_success());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ZeroBytePattern {
    insensitive: bool,
    marks: &'static [(&'static str, u32)],
    message: &'static str,
}

impl<'i> FnOnce<(ParseState<'i>,)> for ZeroBytePattern {
    type Output = ParseResult<'i, (u32, &'i str)>;
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    #[inline]
    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        for (mark, base) in self.marks {
            match Self::parse_byte_base(input, *mark, *base, self.insensitive) {
                Pending(s, v) => return s.finish(v),
                _ => continue,
            }
        }
        StopBecause::missing_character_set(self.message, input.start_offset)?
    }
}

impl ZeroBytePattern {
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn new(marks: &'static [(&'static str, u32)]) -> Self {
        Self { insensitive: false, marks, message: "ZeroBytePattern" }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn with_insensitive(self, insensitive: bool) -> Self {
        Self { insensitive, ..self }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    pub const fn with_message(self, message: &'static str) -> Self {
        Self { message, ..self }
    }
    /// Create a new `ZeroBytePattern` with a leading character and a list of marks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseState, helpers::ZeroBytePattern};
    /// let lower = ParseState::new("0x1234");
    /// assert!(ZeroBytePattern::parse_byte_base(lower, "0X", 16, false).is_failure());
    /// assert!(ZeroBytePattern::parse_byte_base(lower, "0X", 16, true).is_success());
    /// ```
    pub fn parse_byte_base<'i>(
        state: ParseState<'i>,
        mark: &'static str,
        base: u32,
        insensitive: bool,
    ) -> ParseResult<'i, (u32, &'i str)> {
        let (state, _) = match insensitive {
            true => state.match_str_insensitive(mark)?,
            false => state.match_str(mark)?,
        };
        let mut offset = 0;
        for c in state.input.chars() {
            match c {
                // it's a valid digit
                c if c.is_digit(base) => offset += 1,
                _ => break,
            }
        }
        // SAFETY: offset is always less than the length of the residual string
        let str = unsafe { state.input.get_unchecked(..offset) };
        state.advance(offset).finish((base, str))
    }
}
