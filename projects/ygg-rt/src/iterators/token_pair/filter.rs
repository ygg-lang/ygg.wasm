use super::*;

impl<N> Iterator for TokenTreeFilterTag<N>
where
    N: YggdrasilNode,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        match pair.get_tag() {
            Some(s) if s.eq(&self.target) => Some(N::from_pair(pair)),
            _ => self.next(),
        }
    }
}

impl<N> Iterator for TokenTreeFilterRule<N>
where
    N: YggdrasilNode,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        if pair.get_rule().eq(&self.target) {
            return Some(N::from_pair(pair));
        }
        self.next()
    }
}
