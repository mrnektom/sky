#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken,
}

impl ErrorKind {
    // pub(self) fn get_msg(&self) -> &str {
    //     match self {
    //         Self::UnexpectedToken => "Unexpected token recivied",
    //     }
    // }
}
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    index: usize,
    len: usize,
}

impl Error {
    pub fn new(kind: ErrorKind, index: usize, len: usize) -> Self {
        Self { kind, index, len }
    }

    // pub fn get_msg(&self) -> &str {
    //     self.kind.get_msg()
    // }
}
