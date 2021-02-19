use crate::{
    exports::peg::core::combinators::{CstNode, ParseState},
    nodes::ConcreteSyntaxTreeNode,
};
use std::{ops::Add, rc::Rc, string::ParseError};

pub struct InputString {
    pub string: Box<str>,
    pub offset: u32,
}

pub struct WasiParseState {
    /// Rest part of string
    pub input: Rc<InputString>,
    /// Start offset of the string
    pub current_offset: u32,
    pub current_parent: Option<CstNode>,
    pub last_fail: Option<ParseError>,
}

impl WasiParseState {
    pub fn next_char(&self) -> Option<char> {
        self.input.string.chars().next()
    }

    pub fn advance_char(&self, c: char) -> (ParseState, CstNode) {
        let state = Self {
            input: self.input.clone(),
            current_offset: self.current_offset.add(c.len_utf8() as u32),
            current_parent: None,
            last_fail: None,
        };
        (ParseState::new(state), CstNode::new(ConcreteSyntaxTreeNode { parent: self.current_parent.clone(), children: vec![] }))
    }
    pub fn get_start(&self) -> u32 {
        self.current_offset.add(self.input.offset)
    }
}
