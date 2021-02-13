use yggdrasil_error::Validation;

use super::*;

/// According to atomic rules, insert necessary ignore nodes
///
/// | Before | After |
/// | :-: | :-: |
/// | `A B` | `A ~ B` |
/// | `A \| B` | `A \| B` |
/// | `A?` | `A?` |
/// | `A*` | `(~ A)*` |
/// | `A+` | `(~ A)+` |
pub struct InsertIgnore {
    grammar: GrammarInfo,
}

impl Default for InsertIgnore {
    fn default() -> Self {
        Self { grammar: Default::default() }
    }
}

impl CodeOptimizer for InsertIgnore {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        self.grammar = info.clone();
        let mut out = info.clone();
        for rule in out.rules.values_mut() {
            match rule.atomic {
                GrammarAtomic::Atomic => rule.atomic.optimize(),
                GrammarAtomic::Combined => {
                    // println!("Combined: {}", rule.name.text);
                    rule.body.for_each(|e| self.update_node(e));
                    rule.atomic.optimize()
                }
                GrammarAtomic::Optimized => continue,
            }
        }
        Validation::Success { value: out, diagnostics: vec![] }
    }
}

impl InsertIgnore {
    fn update_node(&mut self, info: &mut YggdrasilExpression) {
        match &mut info.body {
            ExpressionBody::Choice(node) => self.update_choice(node),
            ExpressionBody::Concat(node) => self.update_concat(node),
            ExpressionBody::Unary(node) => self.update_unary(node),
            // do nothing
            _ => {}
        }
    }
    fn update_choice(&mut self, info: &mut ChoiceExpression) {
        let mut terms = info.branches.iter().cloned().collect_vec();
        terms.iter_mut().for_each(|e| self.update_node(e));
        info.branches = terms.into_iter().collect();
    }
    fn update_concat(&mut self, info: &mut ConcatExpression) {
        let mut new = Vec::with_capacity(info.sequence.len() * 2);
        for (index, mut old) in info.sequence.iter().cloned().enumerate() {
            if index == 0 {
            }
            else {
                new.push(YggdrasilExpression::ignored())
            }
            self.update_node(&mut old);
            new.push(old)
        }
        // TODO: truncate ignore at begin and end
        info.sequence = new;
    }
    fn update_unary(&mut self, info: &mut UnaryExpression) {
        self.update_node(&mut info.base);
        if info.counter().as_range().end > 1 {
            let head = YggdrasilExpression::ignored();
            info.base = Box::new(head & *info.base.clone());
        }
    }
}
