use std::{ops::Range, fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub fn null() -> Self {
        Self {
            start: 0,
            end: 0,
        }
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start, end
        }
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn from_range(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "[{}]", self.start)
        } else {
            write!(f, "[{}:{}]", self.start, self.end)
        }
    }
}