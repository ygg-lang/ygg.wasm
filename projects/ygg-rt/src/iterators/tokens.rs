use alloc::{rc::Rc, vec::Vec};
use core::{fmt, str};

use super::token_queue::TokenQueue;
use crate::{position, span::InputString, token::Token, YggdrasilRule};

/// An iterator over [`Token`]s. It is created by [`Pair::tokens`] and [`Pairs::tokens`].
///
/// [`Token`]: ../enum.Token.html
/// [`Pair::tokens`]: struct.Pair.html#method.tokens
/// [`Pairs::tokens`]: struct.Pairs.html#method.tokens
#[derive(Clone)]
pub struct Tokens<R> {
    /// # Safety:
    ///
    /// All `QueueableToken`s' `input_pos` must be valid character boundary indices into `input`.
    queue: Rc<Vec<TokenQueue<R>>>,
    input: InputString,
    start: usize,
    end: usize,
}

// TODO(safety): QueueableTokens must be valid indices into input.
pub fn new<R: YggdrasilRule>(queue: Rc<Vec<TokenQueue<R>>>, input: InputString, start: usize, end: usize) -> Tokens<R> {
    if cfg!(debug_assertions) {
        for tok in queue.iter() {
            match *tok {
                TokenQueue::Start { input_offset: input_pos, .. } | TokenQueue::End { input_offset: input_pos, .. } => {
                    // assert!(input.get(input_pos..).is_some(), "💥 UNSAFE `Tokens` CREATED 💥")
                }
            }
        }
    }

    Tokens { queue, input, start, end }
}

impl<'i, R: YggdrasilRule> Tokens<R> {
    fn create_token(&self, index: usize) -> Token<R> {
        match &self.queue[index] {
            TokenQueue::Start { end_token_index, input_offset } => {
                let rule = match &self.queue[*end_token_index] {
                    TokenQueue::End { rule, .. } => rule.clone(),
                    _ => unreachable!(),
                };

                Token::Start {
                    rule,
                    // QueueableTokens are safely created.
                    pos: unsafe { position::Position::new_unchecked(self.input.clone(), *input_offset) },
                }
            }
            TokenQueue::End { rule, input_offset: input_pos, .. } => {
                Token::End {
                    rule: rule.clone(),
                    // QueueableTokens are safely created.
                    pos: unsafe { position::Position::new_unchecked(self.input.clone(), *input_pos) },
                }
            }
        }
    }
}

impl<'i, R: YggdrasilRule> ExactSizeIterator for Tokens<R> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'i, R: YggdrasilRule> Iterator for Tokens<R> {
    type Item = Token<R>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let token = self.create_token(self.start);

        self.start += 1;

        Some(token)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = <Self as ExactSizeIterator>::len(self);
        (len, Some(len))
    }
}

impl<'i, R: YggdrasilRule> DoubleEndedIterator for Tokens<R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }
        let token = self.create_token(self.end - 1);
        self.end -= 1;
        Some(token)
    }
}

impl<'i, R: YggdrasilRule> fmt::Debug for Tokens<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
