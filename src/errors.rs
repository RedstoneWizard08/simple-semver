#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParseError {
    InvalidSemver,
    InvalidNumber,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CheckError {
    SameValue,
    UnknownValues,
}

unsafe impl Send for ParseError {}
unsafe impl Sync for ParseError {}

unsafe impl Send for CheckError {}
unsafe impl Sync for CheckError {}
