use rctree::Node;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    rc::Rc,
};
use yggdrasil_rt::{TokenPair, YggdrasilRule};

pub struct SyntaxData {
    language: &'static str,
    rule: &'static str,
    text: Rc<str>,
    span: Range<usize>,
}

impl Debug for SyntaxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("language", &self.language)
            .field("rule", &self.rule)
            .field("text", &&self.text[self.span.clone()])
            .finish()
    }
}

impl SyntaxData {
    pub fn new<R: YggdrasilRule>(input: Rc<str>, pair: TokenPair<R>) -> Node<SyntaxData> {
        let rule = pair.get_rule();
        let parent = Node::new(SyntaxData {
            language: "json5",
            rule: rule.get_name(),
            text: input.clone(),
            span: pair.get_span().get_range(),
        });
        for child in pair.into_inner() {
            parent.append(SyntaxData::new(input.clone(), child))
        }
        parent
    }
}
