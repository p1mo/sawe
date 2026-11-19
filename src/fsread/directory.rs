use super::{Entry, EntryKind, Status};
use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

pub struct Directory {
    pub entries: Vec<Entry>,
}

impl Directory {

    pub fn walk(root: impl AsRef<Path>, show_root: bool) -> io::Result<Self> {
        let root = root.as_ref();
        if !root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("directory does not exist: {}", root.display()),
            ));
        }

        let root_name = root.file_name().map(PathBuf::from).unwrap_or_default();
        let mut entries = Vec::new();

        Self::scan(root, root, &root_name, show_root, &mut entries)?;
        Ok(Self { entries })
    }

    fn scan(root: &Path, current: &Path, root_name: &Path, show_root: bool, entries: &mut Vec<Entry>) -> io::Result<()> {
        let mut children = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
        children.sort_by_key(|entry| entry.file_name());

        for child in children {
            let path = child.path();
            let relative = path.strip_prefix(root).unwrap_or(&path);
            let display_path = if show_root {
                root_name.join(relative)
            } else {
                relative.to_path_buf()
            };

            let file_type = child.file_type()?;

            if file_type.is_dir() {
                entries.push(Entry::filesystem(display_path, EntryKind::Directory, None, Status::Found));
                Self::scan(root, &path, root_name, show_root, entries)?;
            } else if file_type.is_file() {
                match fs::read(&path) {
                    Ok(bytes) => entries.push(Entry::filesystem(display_path, EntryKind::File, Some(bytes), Status::Found)),
                    Err(_) => entries.push(Entry::filesystem(display_path, EntryKind::File, None, Status::Error)),
                }
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Entry> {
        self.entries.iter()
    }

    /// Parallel iteration. Requires the `fsread-par` feature.
    #[cfg(feature = "fsread-par")]
    pub fn par_iter(&self) -> rayon::slice::Iter<'_, Entry> {
        use rayon::prelude::*;
        self.entries.par_iter()
    }

    pub fn len(&self) -> usize { self.entries.len() }

    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

}
