use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Record<'a> {
    pub line: u64,
    pub offset: u64,
    pub data: Cow<'a, str>,
}
