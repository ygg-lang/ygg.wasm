use std::{ops::Add, rc::Rc};

mod combinators;
mod types;

pub struct PegHost {}

pub struct WasmBase {}

pub enum WasmIterator {
    Ancestors,
    Siblings,
    Previous,
    Following,
    Children,
    Descendants,
}

pub struct WasmNumberNode {}

pub struct WasmStringNode {}

pub struct WasmArrayNode {}

pub struct WasiString {}

pub struct InputString {
    string: Box<str>,
    offset: u32,
}

pub struct WasiParseState {
    /// Rest part of string
    pub input: Rc<InputString>,
    /// Start offset of the string
    pub current_offset: usize,
}

impl WasiParseState {
    pub fn advance_char(&self, c: char) -> Self {
        Self { input: self.input.clone(), current_offset: self.current_offset.add(c.len_utf8()) }
    }
    pub fn get_start(&self) -> u32 {
        self.current_offset.add(self.input.offset)
    }
}

pub struct WasiCharacterMatcher {
    pub c: char,
    pub case_sensitive: bool,
}

pub struct WasiStringMatcher {}
