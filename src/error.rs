use super::*;

use std::path::PathBuf;

use kg_diag::Detail;


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IoError {
    Io {
        kind: std::io::ErrorKind,
    },
    IoPath {
        kind: std::io::ErrorKind,
        op_type: OpType,
        file_type: FileType,
        path: PathBuf,
    },
    CurrentDirGet {
        kind: std::io::ErrorKind,
    },
    CurrentDirSet {
        kind: std::io::ErrorKind,
        path: PathBuf,
    },
    Utf8UnexpectedEof {
        offset: usize,
    },
    Utf8InvalidEncoding {
        offset: usize,
        len: usize,
    },
    Fmt,
}

impl IoError {
    pub fn kind(&self)-> std::io::ErrorKind {
        match *self {
            IoError::Io { kind } => kind,
            IoError::IoPath { kind, .. } => kind,
            IoError::CurrentDirGet { kind, .. } => kind,
            IoError::CurrentDirSet { kind, .. } => kind,
            IoError::Utf8UnexpectedEof { .. } => std::io::ErrorKind::UnexpectedEof,
            IoError::Utf8InvalidEncoding { .. } => std::io::ErrorKind::InvalidData,
            IoError::Fmt => std::io::ErrorKind::Other,
        }
    }
    pub fn file_not_found(path: PathBuf, op_type: OpType)-> IoError {
        IoError::IoPath {
            kind: std::io::ErrorKind::NotFound,
            file_type: FileType::File,
            op_type,
            path,
        }
    }
}

impl Detail for IoError {
    fn code(&self) -> u32 {
        match *self {
            IoError::Io { kind } => 1 + kind as u32,
            IoError::IoPath { kind, .. } => 1 + kind as u32,
            IoError::CurrentDirGet {kind} => 1 + kind as u32,
            IoError::CurrentDirSet {kind, .. } => 1 + kind as u32,
            IoError::Utf8UnexpectedEof { .. } => 20,
            IoError::Utf8InvalidEncoding { .. } => 21,
            IoError::Fmt => 22,
        }
    }
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn kind_str(kind: std::io::ErrorKind) -> &'static str {
            use std::io::ErrorKind;
            match kind {
                ErrorKind::NotFound => "not found",
                ErrorKind::PermissionDenied => "permission denied",
                ErrorKind::ConnectionRefused => "connection refused",
                ErrorKind::ConnectionReset => "connection reset",
                ErrorKind::ConnectionAborted => "connection aborted",
                ErrorKind::NotConnected => "not connected",
                ErrorKind::AddrInUse => "address in use",
                ErrorKind::AddrNotAvailable => "address not available",
                ErrorKind::BrokenPipe => "broken pipe",
                ErrorKind::AlreadyExists => "already exists",
                ErrorKind::WouldBlock => "operation would block",
                ErrorKind::InvalidInput => "invalid input parameter",
                ErrorKind::InvalidData => "invalid data",
                ErrorKind::TimedOut => "timed out",
                ErrorKind::WriteZero => "write zero",
                ErrorKind::Interrupted => "operation interrupted",
                ErrorKind::Other => "other os error",
                ErrorKind::UnexpectedEof => "unexpected end of file",
                _ => unreachable!(),
            }
        }
        match *self {
            IoError::Io { kind } => {
                write!(f, "{}", kind_str(kind))?;
            }
            IoError::IoPath { kind, op_type, file_type, ref path } => {
                write!(f, "cannot {} {} '{}': {}", op_type, file_type, path.display(), kind_str(kind))?;
            }
            IoError::CurrentDirGet { kind } => {
                write!(f, "cannot get current dir: {}", kind_str(kind))?;
            }
            IoError::CurrentDirSet { kind, ref path } => {
                write!(f, "cannot set current dir to {}: {}", path.display(), kind_str(kind))?;
            }
            IoError::Utf8UnexpectedEof { offset, .. } => {
                write!(f, "unexpected end of input at offset {} while decoding utf-8", offset)?;
            }
            IoError::Utf8InvalidEncoding { offset, ..} => {
                write!(f, "invalid utf-8 encoding at offset {}", offset)?;
            }
            IoError::Fmt => {
                write!(f, "formatting error")?;
            }
        }
        Ok(())
    }
}

impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        IoError::Io { kind: err.kind() }
    }
}

impl From<std::io::ErrorKind> for IoError {
    fn from(kind: std::io::ErrorKind) -> Self {
        IoError::Io { kind }
    }
}

impl From<std::fmt::Error> for IoError {
    fn from(_: std::fmt::Error) -> Self {
        IoError::Fmt
    }
}


pub trait ResultExt<T> {
    fn info<P: Into<PathBuf>>(self, path: P, op_type: OpType, file_type: FileType) -> IoResult<T>;
}

impl<T> ResultExt<T> for std::io::Result<T> {
    #[inline]
    fn info<P: Into<PathBuf>>(self, path: P, op_type: OpType, file_type: FileType) -> IoResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                Err(IoError::IoPath {
                    kind: err.kind(),
                    op_type,
                    file_type,
                    path: path.into(),
                })
            }
        }
    }
}
