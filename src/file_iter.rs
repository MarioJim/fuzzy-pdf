use walkdir::{DirEntry, FilterEntry, IntoIter, WalkDir};

/// Encapsulates a `walkdir` iterator
pub enum FileIter {
    AllFilesIter(IntoIter),
    VisibleFilesIter(FilterEntry<IntoIter, for<'r> fn(&'r DirEntry) -> bool>),
}

impl FileIter {
    /// Creates an iterator based on a path and if it should include hidden files
    pub fn new(path: &str, with_hidden_files: bool) -> FileIter {
        if with_hidden_files {
            FileIter::AllFilesIter(WalkDir::new(path).into_iter())
        } else {
            FileIter::VisibleFilesIter(WalkDir::new(path).into_iter().filter_entry(|entry| {
                !entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with('.') && s != "." && s != "..")
                    .unwrap_or(false)
            }))
        }
    }
}

/// Redirect calling `next()` on the enum to the iterator it encapsulates
impl Iterator for FileIter {
    type Item = walkdir::Result<DirEntry>;

    fn next(&mut self) -> Option<walkdir::Result<DirEntry>> {
        match self {
            FileIter::AllFilesIter(it) => it.next(),
            FileIter::VisibleFilesIter(it) => it.next(),
        }
    }
}
