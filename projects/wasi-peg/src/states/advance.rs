use super::*;

/// The advance of the parser.
#[derive(Copy, Clone, Debug)]
pub enum ParseAdvance {
    /// Offset of the advance.
    Offset(usize),
    /// Character of the advance.
    Character(char),
    /// String of the advance.
    String(&'static str),
}

impl From<usize> for ParseAdvance {
    fn from(v: usize) -> Self {
        ParseAdvance::Offset(v)
    }
}

impl From<char> for ParseAdvance {
    fn from(c: char) -> Self {
        ParseAdvance::Character(c)
    }
}

impl From<&'static str> for ParseAdvance {
    fn from(s: &'static str) -> Self {
        ParseAdvance::String(s)
    }
}

impl<'i> ParseState<'i> {
    /// Advance the parser to a new state.
    #[inline]
    pub fn advance<T>(self, term: T) -> ParseState<'i>
    where
        T: Into<ParseAdvance>,
    {
        let offset = term.into().as_offset();
        debug_assert!(offset <= self.input.len(), "offset is out of range");
        let residual = unsafe { self.input.get_unchecked(offset..) };
        ParseState { input: residual, start_offset: self.start_offset + offset, stop_reason: self.stop_reason }
    }
    /// Advance the parser state and return the view of these string.
    #[inline]
    pub fn advance_view(self, offset: usize) -> ParseResult<'i, &'i str> {
        debug_assert!(offset <= self.input.len(), "offset is out of range");
        let view = unsafe { self.input.get_unchecked(..offset) };
        let residual = unsafe { self.input.get_unchecked(offset..) };
        ParseState { input: residual, start_offset: self.start_offset + offset, stop_reason: self.stop_reason }.finish(view)
    }
}

impl ParseAdvance {
    /// Get the offset of the advance.
    #[inline]
    pub const fn as_offset(&self) -> usize {
        match self {
            ParseAdvance::Offset(v) => *v,
            ParseAdvance::Character(v) => v.len_utf8(),
            ParseAdvance::String(v) => v.len(),
        }
    }
}
