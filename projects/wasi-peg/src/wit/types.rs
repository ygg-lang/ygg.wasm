use super::*;
use crate::exports::peg::core::types::*;

impl Guest for PegHost {
    type ParseState = WasiParseState;
}

impl GuestParseState for WasiParseState {}
