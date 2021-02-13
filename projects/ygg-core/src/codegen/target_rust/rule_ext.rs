use std::fmt::Debug;

use convert_case::{Case, Casing};

use yggdrasil_ir::{
    nodes::{ExpressionBody, StreamControl, YggdrasilExpression, YggdrasilOperator},
    rule::GrammarBody,
};

use super::*;

pub(super) trait RuleExt {
    fn parser_expression(&self) -> String;
}

impl RuleExt for GrammarRule {
    fn parser_expression(&self) -> String {
        let mut w = String::new();
        match &self.body {
            // GrammarBody::Empty { .. } => w.push_str("Err(/* empty node */s)"),
            GrammarBody::Class { term } => {
                if let Err(e) = term.write(&mut w, self, true) {
                    w.push_str(&format!("Err(/*{e}*/s)"))
                }
            }
            GrammarBody::Union { branches } => {
                w.push_str("Err(s)");
                for variant in branches {
                    w.push_str(".or_else(|s|");
                    variant.branch.write(&mut w, self, false).unwrap();
                    w.push_str(")");
                }
            }
            GrammarBody::Climb { .. } => w.push_str("Err(/* Climb node */s)"),
            // GrammarBody::TokenSet { .. } => unreachable!("TokenSet is not an expression"),
        }
        w
    }
}

trait NodeExt {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result;
}

impl NodeExt for YggdrasilExpression {
    fn write(&self, w: &mut String, ctx: &GrammarRule, root: bool) -> std::fmt::Result {
        match &self.body {
            ExpressionBody::Choice(v) => {
                w.push_str("Err(s)");
                for pat in &v.branches {
                    w.push_str(".or_else(|s|");
                    pat.write(w, ctx, false)?;
                    w.push_str(")");
                }
            }
            ExpressionBody::Concat(v) => {
                w.push_str("s.sequence(|s|");
                w.push_str("Ok(s)");
                for pat in &v.sequence {
                    w.push_str(".and_then(|s|");
                    pat.write(w, ctx, false)?;
                    w.push_str(")");
                }
                w.push_str(")")
            }
            ExpressionBody::Unary(v) => {
                for o in &v.operators {
                    match o {
                        YggdrasilOperator::Positive => w.push_str("s.lookahead(true,|s|"),
                        YggdrasilOperator::Negative => w.push_str("s.lookahead(false,|s|"),
                        YggdrasilOperator::RepeatsBetween(c) => {
                            if c.is_optional() {
                                write!(w, "s.optional(|s|")?
                            }
                            else {
                                write!(w, "s.repeat({}..{},|s|", c.min, c.max)?
                            }
                        }
                        YggdrasilOperator::Boxing => {
                            todo!()
                        }
                        YggdrasilOperator::Recursive => {
                            todo!()
                        }
                    }
                }
                v.base.write(w, ctx, false)?;
                for _ in &v.operators {
                    w.push_str(")")
                }
            }
            ExpressionBody::Hidden => w.push_str("builtin_ignore(s)"),
            ExpressionBody::Call(v) => write!(w, "Err(/*{}*/s)", v.name.to_string())?,
            ExpressionBody::Rule(r) => {
                let name = format!("parse_{}", r.name.text).to_case(Case::Snake);
                write!(w, "{name}(s)")?
            }
            ExpressionBody::Text(v) if root => write!(w, "s.match_string({:?}, {})", v.text, v.insensitive)?,
            ExpressionBody::Text(v) => write!(w, "builtin_text(s,{:?},{})", v.text, v.insensitive)?,
            ExpressionBody::Regex(r) if root => {
                w.push_str("s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new(");
                write!(w, "{}", r)?;
                w.push_str(").unwrap())})");
            }
            ExpressionBody::Regex(r) => {
                w.push_str("builtin_regex(s,{static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new(");
                write!(w, "{}", r)?;
                w.push_str(").unwrap())})");
            }
            ExpressionBody::CharacterAny if root => w.push_str("s.match_char_if(|_|true)"),
            ExpressionBody::CharacterAny => w.push_str("builtin_any(s)"),
            ExpressionBody::CharacterRange(_) if root => {}
            ExpressionBody::CharacterRange(_) => {}
            ExpressionBody::Boolean(_) if root => {}
            ExpressionBody::Boolean(_) => {}
            ExpressionBody::Integer(_) if root => {}
            ExpressionBody::Integer(_) => {}
            ExpressionBody::Stream(v) => match v {
                StreamControl::StartOfInput => w.push_str("s.start_of_input()"),
                StreamControl::EndOfInput => w.push_str("s.end_of_input()"),
                StreamControl::RestOfLine if root => w.push_str("s.rest_of_line()"),
                StreamControl::RestOfLine => w.push_str("s.rest_of_line()"),
            },
        }
        match &self.tag {
            Some(s) => write!(w, ".and_then(|s| s.tag_node({:?}))", s.text.to_case(Case::Snake))?,
            None => {}
        }
        Ok(())
    }
}
