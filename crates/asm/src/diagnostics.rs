use std::ops::Range;

pub type SourceId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub source: SourceId,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub id: SourceId,
    pub name: String,
    pub source: String,
}
