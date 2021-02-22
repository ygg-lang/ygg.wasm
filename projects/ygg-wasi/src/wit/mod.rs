use crate::syntax_node::{NativeSyntaxData, NativeSyntaxRule};
use rctree::{Ancestors, Children, Descendants, Node};
use std::cell::RefCell;

mod cst;

pub struct YggdrasilHost {}

pub enum NativeSyntaxIterator {
    Ancestors(RefCell<NativeAncestors>),
    Siblings(),
    Previous(),
    Following(),
    Children(Children<NativeSyntaxData>),
    Descendants(RefCell<NativeChildren>),
}

pub struct NativeChildren {
    pub head: Node<NativeSyntaxData>,
    pub current: Option<Node<NativeSyntaxData>>,
    pub reverse: bool,
}

pub struct NativeAncestors {
    pub head: Node<NativeSyntaxData>,
    pub current: Option<Node<NativeSyntaxData>>,
    pub reverse: bool,
}

impl NativeAncestors {
    pub fn new(head: &Node<NativeSyntaxData>, include_self: bool) -> Self {
        Self {
            head: head.clone(),
            current: match include_self {
                true => Some(head.clone()),
                false => head.parent(),
            },
            reverse: false,
        }
    }
}

impl Iterator for NativeAncestors {
    type Item = Node<NativeSyntaxData>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(current) => {
                self.current = current.parent();
                Some(current.clone())
            }
            None => None,
        }
    }
}
