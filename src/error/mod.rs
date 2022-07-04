#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken,
}

impl ErrorKind {
    pub(self) fn get_msg(&self) -> &str {
        match self {
            Self::UnexpectedToken => "Unexpected token recivied",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_msg())
    }
}

#[derive(Debug)]
pub struct Error {
    source: &str,
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n", self.kind))
    }
}

// fn line_by_index(index: usize, source: &str) -> (usize, String) {
//     let mut start = index;
//     let mut end = index;
//     while start > 0 {
//         if let Some("\n") = source.get(start..=start) {
//             break;
//         }
//         start -= 1;
//     }
//     while end < usize::MAX {
//         if let Some("\n") | None = source.get(end..=end) {
//             break;
//         }
//         end += 1;
//     }
//     (index - start, source.get(start..end).unwrap().to_string())
// }

// fn line_number_by_index(mut index: usize, source: &str) -> usize {
//     let mut line = 0;
//     while index > 0 {
//         index -= 1;
//         if source.get(index..=index) == Some("\n") {
//             line += 1;
//         }
//     }
//     line
// }

use std::fmt::Display;
