use alloc::{borrow::Cow, string::String};
use core::ops::Range;

pub trait InputStream {
    fn file_name(&self) -> Option<&str> {
        None
    }
    fn text(&self, span: &Range<usize>) -> Option<Cow<str>>;
}

impl<'i> InputStream for &'i str {
    fn text(&self, span: &Range<usize>) -> Option<Cow<'i, str>> {
        Some(Cow::Borrowed(self.get(span.start..span.end)?))
    }
}

impl InputStream for String {
    fn text(&self, span: &Range<usize>) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.get(span.start..span.end)?))
    }
}
