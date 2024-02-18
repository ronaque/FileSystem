use super::utils;
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

impl Inode {
    pub fn new(mode: u8, name: String) -> Inode {
        if (mode == DIR_MODE) {
            Inode {
                mode,
                size: 0,
                permissions: (true, true),
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                data: InodeData::File(File::new(name)),
            }
        } else {
            Inode {
                mode,
                size: 0,
                permissions: (true, true),
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                data: InodeData::Directory(Directory::new(name)),
            }
        }
    }
}

#[derive(Debug)]
pub struct File {
    name: String,
    data: String,
}

impl File {
    pub fn new(name: String) -> File {
        File {
            name,
            data: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    files: Vec<Inode>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
        }
    }
}
