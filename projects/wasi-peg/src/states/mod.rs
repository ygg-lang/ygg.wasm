use crate::{
    results::StopBecause,
    ParseResult,
    ParseResult::{Pending, Stop},
};
use alloc::{rc::Rc, vec::Vec};
use core::{
    fmt::Debug,
    ops::Range,
    slice::SliceIndex,
    str::pattern::{Pattern, Searcher},
};
#[cfg(feature = "regex")]
use regex::Match;
#[cfg(feature = "regex-automata")]
use regex_automata::MultiMatch;
#[cfg(feature = "ucd-trie")]
use ucd_trie::TrieSetSlice;

pub mod advance;
mod builtin;
pub mod choice;
mod concat;

/// Represent a parsed value
pub type Parsed<T> = (ParseState, T);

/// The state of parsing
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParseState {
    /// Rest part of string
    pub input: Rc<str>,
    /// Start offset of the string
    pub start_offset: usize,
    /// Stop reason
    pub stop_reason: Option<StopBecause>,
}

impl ParseState {
    /// Create a new state
    #[inline(always)]
    pub const fn new(input: &'i str) -> Self {
        Self { input: input, start_offset: 0, stop_reason: None }
    }
    /// Reset the cursor offset
    #[inline(always)]
    pub const fn with_start_offset(mut self, offset: usize) -> Self {
        self.start_offset = offset;
        self
    }
    /// Reset the cursor offset
    #[inline(always)]
    pub const fn end_offset(&self) -> usize {
        self.start_offset + self.input.len()
    }
    /// Finish with given value
    #[inline(always)]
    pub const fn finish<T>(self, value: T) -> ParseResult<'i, T> {
        Pending(self, value)
    }
    /// Check if the string is depleted
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
    /// Get inner error
    #[inline(always)]
    pub const fn get_error(self) -> StopBecause {
        self.stop_reason.unwrap_or(StopBecause::Uninitialized)
    }
    /// Set inner error
    #[inline(always)]
    pub const fn set_error(&mut self, error: StopBecause) {
        self.stop_reason = Some(error);
    }
    /// Get a string view
    #[inline(always)]
    pub fn get_string<R>(&self, range: R) -> Option<&R::Output>
    where
        R: SliceIndex<str>,
    {
        self.input.get(range)
    }
    /// Get nth character
    #[inline(always)]
    pub fn get_character(&self, nth: usize) -> Option<char> {
        self.input.chars().nth(nth)
    }
    /// Get range away from start state
    #[inline(always)]
    pub const fn away_from(&self, start: ParseState) -> Range<usize> {
        start.start_offset..self.start_offset
    }
}
