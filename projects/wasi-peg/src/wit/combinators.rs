use super::*;
use crate::exports::peg::core::combinators::*;

impl Guest for PegHost {
    type CharacterMatcher = WasiCharacterMatcher;
    type TextMatcher = WasiStringMatcher;
}
impl GuestCharacterMatcher for WasiCharacterMatcher {
    fn new(c: char, case_sensitive: bool) -> Self {
        todo!()
    }

    fn match_(&self, state: ParseState) -> ParseState {
        todo!()
    }
}

impl GuestTextMatcher for WasiStringMatcher {
    fn new(s: String, case_sensitive: bool) -> Self {
        todo!()
    }

    fn match_(&self, state: ParseState) -> ParseState {
        todo!()
    }
}
