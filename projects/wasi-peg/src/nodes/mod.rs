use crate::exports::peg::core::combinators::CstNode;

pub struct ConcreteSyntaxTreeNode {
    pub parent: Option<CstNode>,
    pub children: Vec<CstNode>,
}
