use super::*;
use crate::exports::peg::core::combinators::*;
use std::ops::Add;

impl Guest for PegHost {
    type CharacterMatcher = WasiCharacterMatcher;
    type TextMatcher = WasiStringMatcher;
}
impl GuestCharacterMatcher for WasiCharacterMatcher {
    fn new(c: char, case_sensitive: bool) -> Self {
        WasiCharacterMatcher { c, case_sensitive }
    }

    fn match_(&self, state: ParseState) -> Result<(ParseState, char), ParseError> {
        let state: WasiParseState = state.into_inner::<WasiParseState>();
        match state.input.starts_with(self.c) {
            true => state.advance_char(self.c),
            false => {}
        }

        Ok((ParseState::new(state), 'a'))
    }
}

impl GuestTextMatcher for WasiStringMatcher {
    fn new(s: String, case_sensitive: bool) -> Self {
        todo!()
    }

    fn match_(&self, state: ParseState) -> Result<(ParseState, String), ParseError> {
        todo!()
    }
}
