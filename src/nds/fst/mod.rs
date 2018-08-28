/// Represents an entry in the File Allocation Table.
pub struct FatEntry {
    /// The offset to the start of the file relative to the ROM start.
    offset: u32,
    /// Length of the file.
    len: u32,
}

impl FatEntry {
    pub fn new(offset: u32, len: u32) -> Self {
        Self {
            offset,
            len,
        }
    }
}

/// Represents an entry in the File System Table.
pub struct FstEntry {
    /// The id of the FST node.
    id: u16,
    /// The name of the file or folder.
    name: String,
    /// If the entry is a directory, it will have child entries.
    children: Option<Vec<FstEntry>>,
    /// If the entry is a file, it will have an allocation table entry.
    alloc: Option<FatEntry>,
}

impl FstEntry {
    pub fn new(id: u16, name: &str, children: Option<Vec<FstEntry>>, alloc: Option<FatEntry>) -> Self {
        Self {
            id,
            name: name.into(),
            children,
            alloc,
        }
    }
}

/// Represents the File System Table. Holds the root node of a tree
/// which holds all the files and directories.
pub struct Fst {
    /// Root directory of the FST.
    root: FstEntry,
}

impl Fst {
    /// Creates a File System Table given the raw ROM data.
    /// In order to get all the information to find files,
    /// the File Allocation Table is passed in as well.
    pub fn new(fst: &[u8], fat: &[u8]) -> Self {
        Self {
            root: FstEntry::new(0, ".", None, None),
        }
    }
}