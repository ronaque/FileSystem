pub const DIR_MODE: u8 = 0;
pub const FILE_MODE: u8 = 1;

#[derive(Debug)]
pub enum InodeData {
    File(File),
    Directory(Directory),
}

#[derive(Debug)]
pub struct Inode {
    mode: u8,                  // file or directory
    size: u64,                 // in bytes
    permissions: (bool, bool), // (read, write)
    created_at: Option<u64>,
    updated_at: Option<u64>,
    accessed_at: Option<u64>,
    data: InodeData,
}

#[derive(Debug)]
pub struct File {
    name: String,
    data: String,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    files: Vec<Inode>,
}
