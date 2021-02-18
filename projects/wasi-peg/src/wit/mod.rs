use std::rc::Rc;

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

pub struct WasiParseState {
    /// Rest part of string
    pub input: Rc<str>,
    /// Start offset of the string
    pub start_offset: usize,
}

pub struct WasiCharacterMatcher {}

pub struct WasiStringMatcher {}
