use super::*;
use crate::exports::yggdrasil::json::ast::*;

impl Guest for JsonHost {
    type JsonBase = WasmBase;
    type JsonIterator = WasmIterator;
    type JsonNumberNode = WasmNumberNode;
    type JsonStringNode = WasmStringNode;
    type JsonArrayNode = WasmArrayNode;
}

impl GuestJsonBase for WasmBase {
    fn is_leaf(&self) -> bool {
        unimplemented!()
    }

    fn get_hash(&self) -> u64 {
        unimplemented!()
    }

    fn get_range(&self) -> TextRange {
        unimplemented!()
    }

    fn get_token(&self) -> JsonToken {
        unimplemented!()
    }

    fn get_text(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get_parent(&self) -> Option<JsonType> {
        unimplemented!()
    }

    fn get_ancestors(&self, include_self: bool) -> JsonIterator {
        unimplemented!()
    }

    fn get_last(&self) -> Option<JsonType> {
        unimplemented!()
    }

    fn get_last_iterator(&self, include_self: bool) -> JsonIterator {
        unimplemented!()
    }

    fn get_next(&self) -> Option<JsonType> {
        unimplemented!()
    }

    fn get_next_iterator(&self, include_self: bool) -> JsonIterator {
        unimplemented!()
    }

    fn get_siblings(&self) -> JsonIterator {
        unimplemented!()
    }

    fn get_children(&self) -> JsonIterator {
        unimplemented!()
    }

    fn get_descendants(&self, depth_first: bool) -> JsonIterator {
        unimplemented!()
    }
}

impl GuestJsonIterator for WasmIterator {
    fn last(&self) -> Option<JsonBase> {
        unimplemented!()
    }

    fn next(&self) -> Option<JsonBase> {
        unimplemented!()
    }

    fn skip(&self, count: u32) {
        unimplemented!()
    }

    fn move_head(&self) {
        unimplemented!()
    }

    fn move_tail(&self) {
        unimplemented!()
    }

    fn reverse(&self) {
        unimplemented!()
    }
}

impl GuestJsonArrayNode for WasmArrayNode {
    fn ctor(base: JsonBase) -> Result<JsonArrayNode, ParseError> {
        todo!()
    }

    fn parse(text: String, offset: u32) -> Result<JsonArrayNode, ParseError> {
        todo!()
    }

    fn item(&self) -> Vec<JsonNode> {
        todo!()
    }
}

impl GuestJsonStringNode for WasmStringNode {}

impl GuestJsonNumberNode for WasmNumberNode {}
