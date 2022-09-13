#[derive(Debug, Clone)]
pub(crate) struct LineNumber {
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Range {
    pub start: usize,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum DiffU<'a> {
    CaretPos {
        left: Range,
        right: Range,
    },
    Addition(&'a str),
    Deletion(&'a str),
    Display(&'a str),
}
