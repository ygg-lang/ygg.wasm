use super::*;
#[automatically_derived]
impl YggdrasilNode for RootNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            statement: pair.take_tagged_items::<StatementNode>(Cow::Borrowed("statement")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RootNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Root)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::ClassStatement(s) => s.get_range(),
            Self::GrammarStatement(s) => s.get_range(),
            Self::GroupStatement(s) => s.get_range(),
            Self::UnionStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ClassStatementNode>(Cow::Borrowed("class_statement")) {
            return Ok(Self::ClassStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GrammarStatementNode>(Cow::Borrowed("grammar_statement")) {
            return Ok(Self::GrammarStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupStatementNode>(Cow::Borrowed("group_statement")) {
            return Ok(Self::GroupStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<UnionStatementNode>(Cow::Borrowed("union_statement")) {
            return Ok(Self::UnionStatement(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Statement, _span))
    }
}
#[automatically_derived]
impl FromStr for StatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Statement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_dict: pair.take_tagged_one::<GrammarDictNode>(Cow::Borrowed("grammar_dict"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GrammarStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarTermNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarPair(s) => s.get_range(),
            Self::GrammarTerm1 => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<GrammarPairNode>(Cow::Borrowed("grammar_pair")) {
            return Ok(Self::GrammarPair(s));
        }
        if let Some(_) = pair.find_first_tag("grammar_term_1") {
            return Ok(Self::GrammarTerm1);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarTerm, _span))
    }
}
#[automatically_derived]
impl FromStr for GrammarTermNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarTerm)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarPairNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_value: pair.take_tagged_one::<GrammarValueNode>(Cow::Borrowed("grammar_value"))?,
            key: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("key"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GrammarPairNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarPair)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarValueNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarDict(s) => s.get_range(),
            Self::GrammarList(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<GrammarDictNode>(Cow::Borrowed("grammar_dict")) {
            return Ok(Self::GrammarDict(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GrammarListNode>(Cow::Borrowed("grammar_list")) {
            return Ok(Self::GrammarList(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNormalNode>(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringRawNode>(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarValue, _span))
    }
}
#[automatically_derived]
impl FromStr for GrammarValueNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarValue)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarDictNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_term: pair
                .take_tagged_items::<GrammarTermNode>(Cow::Borrowed("grammar_term"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GrammarDictNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarDict)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarListNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_value: pair
                .take_tagged_items::<GrammarValueNode>(Cow::Borrowed("grammar_value"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GrammarListNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarList)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GrammarListTermsNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_value: pair
                .take_tagged_items::<GrammarValueNode>(Cow::Borrowed("grammar_value"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GrammarListTermsNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarListTerms)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            decorator_call: pair
                .take_tagged_items::<DecoratorCallNode>(Cow::Borrowed("decorator_call"))
                .collect::<Result<Vec<_>, _>>()?,
            modifier_call: pair
                .take_tagged_items::<ModifierCallNode>(Cow::Borrowed("modifier_call"))
                .collect::<Result<Vec<_>, _>>()?,
            op_remark: pair.take_tagged_option::<OpRemarkNode>(Cow::Borrowed("op_remark")),
            cast: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("cast")),
            name: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OpRemarkNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for OpRemarkNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::OP_REMARK)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for UnionStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            decorator_call: pair
                .take_tagged_items::<DecoratorCallNode>(Cow::Borrowed("decorator_call"))
                .collect::<Result<Vec<_>, _>>()?,
            modifier_call: pair
                .take_tagged_items::<ModifierCallNode>(Cow::Borrowed("modifier_call"))
                .collect::<Result<Vec<_>, _>>()?,
            op_remark: pair.take_tagged_option::<OpRemarkNode>(Cow::Borrowed("op_remark")),
            union_block: pair.take_tagged_one::<UnionBlockNode>(Cow::Borrowed("union_block"))?,
            name: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for UnionBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            union_branch: pair
                .take_tagged_items::<UnionBranchNode>(Cow::Borrowed("union_branch"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for UnionBranchNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            branch_tag: pair.take_tagged_option::<BranchTagNode>(Cow::Borrowed("branch_tag")),
            expression_hard: pair.take_tagged_one::<ExpressionHardNode>(Cow::Borrowed("expression_hard"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionBranchNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBranch)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BranchTagNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            right_associativity: pair.take_tagged_option::<RightAssociativityNode>(Cow::Borrowed("right_associativity")),
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for BranchTagNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::BranchTag)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RightAssociativityNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for RightAssociativityNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RightAssociativity)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            decorator_call: pair
                .take_tagged_items::<DecoratorCallNode>(Cow::Borrowed("decorator_call"))
                .collect::<Result<Vec<_>, _>>()?,
            group_block: pair.take_tagged_one::<GroupBlockNode>(Cow::Borrowed("group_block"))?,
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            modifier_call: pair
                .take_tagged_items::<ModifierCallNode>(Cow::Borrowed("modifier_call"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            group_pair: pair.take_tagged_items::<GroupPairNode>(Cow::Borrowed("group_pair")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupPairNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupPairNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupPair)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DecoratorCallNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            call_body: pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
            decorator_name: pair.take_tagged_one::<DecoratorNameNode>(Cow::Borrowed("decorator_name"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for DecoratorCallNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::DecoratorCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DecoratorNameNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for DecoratorNameNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::DecoratorName)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for FunctionCallNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            call_body: pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
            function_name: pair.take_tagged_one::<FunctionNameNode>(Cow::Borrowed("function_name"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for FunctionCallNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for FunctionNameNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for FunctionNameNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionName)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CallBodyNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for CallBodyNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::CallBody)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_hard: pair
                .take_tagged_items::<ExpressionHardNode>(Cow::Borrowed("expression_hard"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionHardNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_soft: pair
                .take_tagged_items::<ExpressionSoftNode>(Cow::Borrowed("expression_soft"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionHardNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionHard)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionSoftNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_tag: pair
                .take_tagged_items::<ExpressionTagNode>(Cow::Borrowed("expression_tag"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionSoftNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionSoft)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionTagNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            term: pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionTagNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionTag)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TermNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            prefix: pair.take_tagged_items::<PrefixNode>(Cow::Borrowed("prefix")).collect::<Result<Vec<_>, _>>()?,
            suffix: pair.take_tagged_items::<SuffixNode>(Cow::Borrowed("suffix")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for TermNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Term)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PrefixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Negative => Range::default(),
            Self::Positive => Range::default(),
            Self::Remark => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("negative") {
            return Ok(Self::Negative);
        }
        if let Some(_) = pair.find_first_tag("positive") {
            return Ok(Self::Positive);
        }
        if let Some(_) = pair.find_first_tag("remark") {
            return Ok(Self::Remark);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Prefix, _span))
    }
}
#[automatically_derived]
impl FromStr for PrefixNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Prefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Many => Range::default(),
            Self::Many1 => Range::default(),
            Self::Optional => Range::default(),
            Self::Range(s) => s.get_range(),
            Self::RangeExact(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("many") {
            return Ok(Self::Many);
        }
        if let Some(_) = pair.find_first_tag("many_1") {
            return Ok(Self::Many1);
        }
        if let Some(_) = pair.find_first_tag("optional") {
            return Ok(Self::Optional);
        }
        if let Ok(s) = pair.take_tagged_one::<RangeNode>(Cow::Borrowed("range")) {
            return Ok(Self::Range(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RangeExactNode>(Cow::Borrowed("range_exact")) {
            return Ok(Self::RangeExact(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Suffix, _span))
    }
}
#[automatically_derived]
impl FromStr for SuffixNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Suffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Boolean(s) => s.get_range(),
            Self::Category(s) => s.get_range(),
            Self::EscapedUnicode(s) => s.get_range(),
            Self::FunctionCall(s) => s.get_range(),
            Self::GroupExpression(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
            Self::Integer(s) => s.get_range(),
            Self::RegexEmbed(s) => s.get_range(),
            Self::RegexRange(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<CategoryNode>(Cow::Borrowed("category")) {
            return Ok(Self::Category(s));
        }
        if let Ok(s) = pair.take_tagged_one::<EscapedUnicodeNode>(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one::<FunctionCallNode>(Cow::Borrowed("function_call")) {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupExpressionNode>(Cow::Borrowed("group_expression")) {
            return Ok(Self::GroupExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexEmbedNode>(Cow::Borrowed("regex_embed")) {
            return Ok(Self::RegexEmbed(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexRangeNode>(Cow::Borrowed("regex_range")) {
            return Ok(Self::RegexRange(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNormalNode>(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringRawNode>(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Atomic)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupExpressionNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupExpressionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupExpression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringRawNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            string_raw_text: pair.take_tagged_one::<StringRawTextNode>(Cow::Borrowed("string_raw_text"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for StringRawNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringRaw)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringRawTextNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for StringRawTextNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringRawText)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNormalNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            string_item: pair
                .take_tagged_items::<StringItemNode>(Cow::Borrowed("string_item"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for StringNormalNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringNormal)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringItemNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedCharacter(s) => s.get_range(),
            Self::EscapedUnicode(s) => s.get_range(),
            Self::TextAny(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<EscapedCharacterNode>(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one::<EscapedUnicodeNode>(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextAnyNode>(Cow::Borrowed("text_any")) {
            return Ok(Self::TextAny(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::StringItem, _span))
    }
}
#[automatically_derived]
impl FromStr for StringItemNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EscapedUnicodeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            hex: pair.take_tagged_one::<HexNode>(Cow::Borrowed("hex"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for EscapedUnicodeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::EscapedUnicode)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EscapedCharacterNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for EscapedCharacterNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::EscapedCharacter)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for HexNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for HexNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::HEX)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextAnyNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for TextAnyNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::TextAny)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexEmbedNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            regex_item: pair.take_tagged_items::<RegexItemNode>(Cow::Borrowed("regex_item")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RegexEmbedNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexEmbed)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexItemNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedCharacter(s) => s.get_range(),
            Self::RegexCharacter(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<EscapedCharacterNode>(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexCharacterNode>(Cow::Borrowed("regex_character")) {
            return Ok(Self::RegexCharacter(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::RegexItem, _span))
    }
}
#[automatically_derived]
impl FromStr for RegexItemNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexCharacterNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for RegexCharacterNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexCharacter)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexRangeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            regex_negative: pair.take_tagged_option::<RegexNegativeNode>(Cow::Borrowed("regex_negative")),
            text: pair.get_string(),
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RegexRangeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexRange)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexNegativeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for RegexNegativeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexNegative)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CategoryNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            group: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("group")),
            script: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("script"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for CategoryNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Category)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathFreeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathFreeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::NamepathFree)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Namepath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::False => Range::default(),
            Self::True => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("false") {
            return Ok(Self::False);
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Boolean)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IntegerNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for IntegerNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Integer)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeExactNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            integer: pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeExactNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RangeExact)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            max: pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("max")),
            min: pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("min")),
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Range)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ModifierCallNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ModifierCallNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ModifierCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OpCategoryNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for OpCategoryNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::OP_CATEGORY)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwExternalNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::External => Range::default(),
            Self::Inspector => Range::default(),
            Self::Parser => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("external") {
            return Ok(Self::External);
        }
        if let Some(_) = pair.find_first_tag("inspector") {
            return Ok(Self::Inspector);
        }
        if let Some(_) = pair.find_first_tag("parser") {
            return Ok(Self::Parser);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::KW_EXTERNAL, _span))
    }
}
#[automatically_derived]
impl FromStr for KwExternalNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_EXTERNAL)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwGrammarNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwGrammarNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_GRAMMAR)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwImportNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwImportNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_IMPORT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClassNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwClassNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_CLASS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwUnionNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwUnionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_UNION)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwGroupNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwGroupNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_GROUP)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClimbNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwClimbNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_CLIMB)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwMacroNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for KwMacroNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_MACRO)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::WhiteSpace)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CommentNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for CommentNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Comment)?)
    }
}
