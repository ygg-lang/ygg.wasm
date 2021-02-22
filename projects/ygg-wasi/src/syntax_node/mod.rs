use crate::exports::peg::core::cst::{SnytaxFlags, SyntaxRule};
use rctree::Node;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    rc::Rc,
};
use yggdrasil_rt::{TokenPair, YggdrasilRule};

#[derive(Debug, Clone)]
pub struct NativeLanguage {
    pub name: &'static str,
    pub glob: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct NativeSyntaxRule {
    pub language: NativeLanguage,
    pub flags: SnytaxFlags,
    pub tag: String,
    pub name: &'static str,
    pub styles: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct NativeSyntaxData {
    pub rule: NativeSyntaxRule,
    pub text: Rc<str>,
    pub span: Range<usize>,
}

impl NativeSyntaxData {
    pub fn new<R: YggdrasilRule + 'static>(
        input: Rc<str>,
        pair: TokenPair<R>,
        language: &NativeLanguage,
    ) -> Node<NativeSyntaxData> {
        let rule = pair.get_rule();
        let parent = Node::new(NativeSyntaxData {
            rule: NativeSyntaxRule {
                language: language.clone(),
                flags: SnytaxFlags::empty(),
                tag: pair.get_tag().map(|x| x.to_owned()).unwrap_or_default(),
                name: rule.get_name(),
                styles: vec![rule.get_style().to_string()],
            },
            text: input.clone(),
            span: pair.get_span().get_range(),
        });
        for child in pair.into_inner() {
            parent.append(NativeSyntaxData::new(input.clone(), child, language))
        }
        parent
    }
}
