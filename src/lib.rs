#![feature(box_syntax, specialization)]

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate kg_diag;
#[macro_use]
extern crate kg_display_derive;

#[cfg(test)]
extern crate tempfile;

use kg_diag::*;

pub mod error;
pub mod fs;
mod reader;

pub use kg_diag::{Position, Quote};
pub use self::reader::{Reader, ByteReader, CharReader, MemByteReader, MemCharReader};
pub use self::fs::{FileBuffer, OpType, FileType};

pub use self::error::{IoError, ResultExt};
pub type IoResult<T> = std::result::Result<T, IoError>;
pub type ParseResult<T> = std::result::Result<T, ParseDiag>;

