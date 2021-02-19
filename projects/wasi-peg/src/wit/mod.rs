use std::{
    ops::{Add, RangeInclusive},
    rc::Rc,
};

mod combinators;
mod types;

pub struct PegHost {}

pub struct WasiCharacterMatcher {
    pub range: RangeInclusive<char>,
    pub case_sensitive: bool,
}

pub struct WasiStringMatcher {
    pub s: String,
    pub case_sensitive: bool,
}
