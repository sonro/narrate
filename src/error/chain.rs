use std::error::Error as StdError;

use crate::Chain;

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn StdError + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for Chain<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl ExactSizeIterator for Chain<'_> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> From<anyhow::Chain<'a>> for Chain<'a> {
    fn from(inner: anyhow::Chain<'a>) -> Self {
        Self { inner }
    }
}
