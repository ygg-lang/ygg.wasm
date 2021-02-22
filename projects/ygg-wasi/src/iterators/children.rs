use super::*;

impl NativeChildren {
    pub fn new(parent: &Node<NativeSyntaxData>) -> MaybeReversed<Self> {
        MaybeReversed { iterator: Self { parent: parent.clone(), current: parent.first_child() }, reversed: false }
    }
}

impl Iterator for NativeChildren {
    type Item = Node<NativeSyntaxData>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(s) => {
                let ret = s.clone();
                self.current = s.next_sibling();
                Some(ret)
            }
            None => None,
        }
    }
}
impl DoubleEndedIterator for NativeChildren {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(s) => {
                let ret = s.clone();
                self.current = s.previous_sibling();
                Some(ret)
            }
            None => None,
        }
    }
}
