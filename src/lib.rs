#![feature(specialization)]

#[macro_use]
extern crate kg_display_derive;
#[macro_use]
extern crate serde_derive;


use kg_diag::*;
pub use kg_diag::{Position, Quote};

pub use self::error::{IoError, ResultExt};
pub use self::fs::{FileBuffer, FileType, OpType};
pub use self::reader::{ByteReader, CharReader, MemByteReader, MemCharReader, Reader};

pub mod error;
pub mod fs;
mod reader;

pub type IoResult<T> = std::result::Result<T, IoError>;
pub type ParseResult<T> = std::result::Result<T, ParseDiag>;

