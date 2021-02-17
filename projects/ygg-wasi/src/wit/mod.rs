mod ast;

pub struct JsonHost {}

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
