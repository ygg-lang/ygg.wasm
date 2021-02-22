use crate::syntax_node::NativeSyntaxData;
use rctree::{Children, Node};
use std::cell::RefCell;

mod ancestors;
mod children;

pub enum NativeSyntaxIterator {
    Ancestors(RefCell<MaybeReversed<NativeAncestors>>),
    Siblings(),
    Previous(),
    Following(),
    Children(RefCell<MaybeReversed<NativeChildren>>),
    Descendants(RefCell<NativeChildren>),
}
pub struct NativeChildren {
    pub parent: Node<NativeSyntaxData>,
    pub current: Option<Node<NativeSyntaxData>>,
}

pub struct NativeAncestors {
    // 0 is head
    pub remember: Vec<Node<NativeSyntaxData>>,
    pub current: usize,
}

pub struct MaybeReversed<T> {
    iterator: T,
    reversed: bool,
}

impl<T> MaybeReversed<T>
where
    T: DoubleEndedIterator,
{
    pub fn forward(&mut self) -> Option<Node<NativeSyntaxData>> {
        if self.reversed { self.iterator.next_back() } else { self.iterator.next() }
    }
    pub fn backward(&mut self) -> Option<Node<NativeSyntaxData>> {
        if self.reversed { self.iterator.next() } else { self.iterator.next_back() }
    }
    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
    }
}
