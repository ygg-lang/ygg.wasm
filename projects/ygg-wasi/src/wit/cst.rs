use super::*;
use std::ops::Deref;

use crate::exports::peg::core::cst::*;

impl Guest for YggdrasilHost {
    type SyntaxRule = NativeSyntaxRule;
    type SyntaxNode = Node<NativeSyntaxData>;
    type SyntaxIterator = NativeSyntaxIterator;
}

impl GuestSyntaxRule for NativeSyntaxRule {}

impl GuestSyntaxNode for Node<NativeSyntaxData> {
    fn is_leaf(&self) -> bool {
        todo!()
    }

    fn get_hash(&self) -> u64 {
        todo!()
    }

    fn get_range(&self) -> TextRange {
        let data = self.borrow();
        let range = data.span.clone();
        TextRange { head_offset: range.start as u32, tail_offset: range.end as u32 }
    }

    fn get_rule(&self) -> SyntaxRule {
        todo!()
    }

    fn get_text(&self) -> String {
        let data = self.borrow();
        let range = data.span.clone();
        match data.text.get(range) {
            Some(s) => s.to_string(),
            None => {
                panic!("out of range")
            }
        }
    }

    fn get_parent(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.parent()?))
    }

    fn get_ancestors(&self, include_self: bool) -> SyntaxIterator {
        SyntaxIterator::new(NativeSyntaxIterator::Ancestors(RefCell::new(NativeAncestors::new(self, include_self))))
    }

    fn get_last(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.previous_sibling()?))
    }

    fn get_last_iterator(&self, include_self: bool) -> SyntaxIterator {
        todo!()
    }

    fn get_next(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.next_sibling()?))
    }

    fn get_next_iterator(&self, include_self: bool) -> SyntaxIterator {
        todo!()
    }

    fn get_siblings(&self) -> SyntaxIterator {
        todo!()
    }

    fn get_children(&self) -> SyntaxIterator {
        SyntaxIterator::new(NativeSyntaxIterator::Children(self.children()))
    }

    fn get_descendants(&self, depth_first: bool) -> SyntaxIterator {
        SyntaxIterator::new(NativeSyntaxIterator::Descendants(self.descendants()))
    }
}
impl GuestSyntaxIterator for NativeSyntaxIterator {
    fn last(&self) -> Option<SyntaxNode> {
        todo!()
    }

    fn next(&self) -> Option<SyntaxNode> {
        match self {
            NativeSyntaxIterator::Ancestors(ancestors) => Some(SyntaxNode::new(ancestors.borrow_mut().next()?)),
            NativeSyntaxIterator::Siblings() => {
                todo!()
            }
            NativeSyntaxIterator::Previous() => {
                todo!()
            }
            NativeSyntaxIterator::Following() => {
                todo!()
            }
            NativeSyntaxIterator::Children(_) => {
                todo!()
            }
            NativeSyntaxIterator::Descendants(_) => {
                todo!()
            }
        }
    }

    fn move_head(&self) {
        todo!()
    }

    fn move_tail(&self) {
        todo!()
    }

    fn skip(&self, count: u32) {
        todo!()
    }

    fn reverse(&self) {
        todo!()
    }
}
