use super::*;
use crate::{exports::peg::core::types::*, nodes::ConcreteSyntaxTreeNode, states::WasiParseState};

impl Guest for PegHost {
    type ParseState = WasiParseState;
    type CstNode = ConcreteSyntaxTreeNode;
}

impl GuestParseState for WasiParseState {
    fn get_input(&self) -> String {
        todo!()
    }

    fn get_rest(&self) -> String {
        todo!()
    }
}

impl GuestCstNode for ConcreteSyntaxTreeNode {}
