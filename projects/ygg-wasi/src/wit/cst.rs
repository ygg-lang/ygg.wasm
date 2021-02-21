use super::*;
use crate::exports::yggdrasil::json::ast::*;

impl Guest for YggdrasilHost {
    type JsonNumberNode = WasmNumberNode;
    type JsonStringNode = WasmStringNode;
    type JsonArrayNode = WasmArrayNode;
}

impl GuestJsonArrayNode for WasmArrayNode {
    fn ctor(base: JsonBase) -> Result<JsonArrayNode, ParseError> {
        todo!()
    }

    fn parse_string(text: String, offset: u32) -> Result<JsonArrayNode, ParseError> {
        todo!()
    }

    fn get_super(&self) -> JsonBase {
        todo!()
    }

    fn item(&self) -> Vec<JsonNode> {
        todo!()
    }
}

impl GuestJsonStringNode for WasmStringNode {}

impl GuestJsonNumberNode for WasmNumberNode {}
