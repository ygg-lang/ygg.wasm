use super::*;
use crate::{
    exports::peg::core::{combinators::*, types::MissingCharacter},
    states::WasiParseState,
};
use std::ops::Add;

impl Guest for PegHost {
    type CharacterMatcher = WasiCharacterMatcher;
    type TextMatcher = WasiStringMatcher;
}

impl GuestCharacterMatcher for WasiCharacterMatcher {
    fn new(c: char, case_sensitive: bool) -> Self {
        WasiCharacterMatcher { range: c..=c, case_sensitive }
    }

    fn parse(&self, state: ParseState) -> Result<(ParseState, CstNode), ParseError> {
        let state: WasiParseState = state.into_inner::<WasiParseState>();
        match state.next_char() {
            Some(s) => match self.range.contains(&s) {
                true => Ok(state.advance_char(s)),
                false => Err(ParseError::MissingCharacter(MissingCharacter { c: s, position: 0 })),
            },
            None => Err(ParseError::MissingCharacter(MissingCharacter { c: '?', position: 0 })),
        }
    }
}

impl GuestTextMatcher for WasiStringMatcher {
    fn new(s: String, case_sensitive: bool) -> Self {
        todo!()
    }

    fn parse(&self, state: ParseState) -> Result<(ParseState, CstNode), ParseError> {
        todo!()
    }
}
