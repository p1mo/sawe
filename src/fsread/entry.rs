use super::{compression, Codec};
use std::{
    io,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Embedded,
    Found,
    Error,
}

struct Content {
    stored: Arc<[u8]>,
    codec: Codec,
    decoded: OnceLock<Result<Arc<[u8]>, Arc<str>>>,
}

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub kind: EntryKind,
    pub status: Status,
    content: Option<Arc<Content>>,
}

impl Entry {
    pub fn embedded_dir(path: PathBuf) -> Self {
        Self { path, kind: EntryKind::Directory, status: Status::Embedded, content: None }
    }

    pub fn embedded_file(path: PathBuf, bytes: &'static [u8], codec: Codec) -> Self {
        Self {
            path,
            kind: EntryKind::File,
            status: Status::Embedded,
            content: Some(Arc::new(Content {
                stored: bytes.into(),
                codec,
                decoded: OnceLock::new(),
            })),
        }
    }

    pub(crate) fn filesystem(path: PathBuf, kind: EntryKind, bytes: Option<Vec<u8>>, status: Status) -> Self {
        Self {
            path,
            kind,
            status,
            content: bytes.map(|bytes| Arc::new(Content {
                stored: bytes.into(),
                codec: Codec::None,
                decoded: OnceLock::new(),
            })),
        }
    }

    pub fn is_file(&self) -> bool { self.kind == EntryKind::File }
    pub fn is_dir(&self) -> bool { self.kind == EntryKind::Directory }

    pub fn codec(&self) -> Codec {
        self.content.as_ref().map_or(Codec::None, |c| c.codec)
    }

    pub fn is_compressed(&self) -> bool { self.codec() != Codec::None }

    /// Returns the stored bytes. These may be compressed.
    pub fn stored_bytes(&self) -> Option<&[u8]> {
        self.content.as_ref().map(|c| c.stored.as_ref())
    }

    /// Returns original bytes, transparently decompressing and caching if needed.
    pub fn bytes(&self) -> io::Result<Option<Arc<[u8]>>> {
        let Some(content) = &self.content else { return Ok(None) };

        if content.codec == Codec::None {
            return Ok(Some(content.stored.clone()));
        }

        let decoded = content.decoded.get_or_init(|| {
            compression::decompress(&content.stored, content.codec)
                .map(Arc::<[u8]>::from)
                .map_err(|e| Arc::<str>::from(e.to_string()))
        });

        decoded.clone().map(Some).map_err(|message| {
            io::Error::new(io::ErrorKind::InvalidData, message.to_string())
        })
    }

    /// Number of bytes stored in the entry (compressed size when compressed).
    pub fn size(&self) -> usize {
        self.content.as_ref().map_or(0, |c| c.stored.len())
    }
}
