#[derive(Debug, Clone)]
pub(crate) struct LineNumber {
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Range {
    pub(crate) start: usize,
    pub(crate) count: usize,
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

impl<'a> ToString for DiffU<'a> {
    fn to_string(&self) -> String {
        match self {
            DiffU::CaretPos { left, right } => format!("@@ -{},{} +{},{} @@", left.start, left.count, right.start, right.count),
            DiffU::Addition(s) => format!("+{}", s),
            DiffU::Deletion(s) => format!("-{}", s),
            DiffU::Display(s) => {
                if s.len() == 0 {
                    String::with_capacity(0)
                } else {
                    format!(" {}", s)
                }
            }
        }
    }
}
