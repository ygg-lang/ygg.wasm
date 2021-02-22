use rctree::Node;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    rc::Rc,
};
use yggdrasil_rt::{TokenPair, YggdrasilRule};

pub struct NativeSyntaxRule {}

pub struct NativeSyntaxData {
    pub language: &'static str,
    pub rule: &'static str,
    pub text: Rc<str>,
    pub span: Range<usize>,
}

impl Debug for NativeSyntaxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("language", &self.language)
            .field("rule", &self.rule)
            .field("text", &&self.text[self.span.clone()])
            .finish()
    }
}

impl NativeSyntaxData {
    pub fn new<R: YggdrasilRule>(input: Rc<str>, pair: TokenPair<R>) -> Node<NativeSyntaxData> {
        let rule = pair.get_rule();
        let parent = Node::new(NativeSyntaxData {
            language: "json5",
            rule: rule.get_name(),
            text: input.clone(),
            span: pair.get_span().get_range(),
        });
        for child in pair.into_inner() {
            parent.append(NativeSyntaxData::new(input.clone(), child))
        }
        parent
    }
}
