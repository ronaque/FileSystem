use std::io::{Stdout, Write};
use std::mem::size_of;
use super::utils;
pub const DIR_MODE: u8 = 0;
pub const FILE_MODE: u8 = 1;

const ROOT_INODE: u64 = 0;
static mut INODE_SERIAL_NUMER: u64 = 0;

#[derive(Debug, Clone)]
pub enum InodeData {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone)]
pub struct Inode {
    mode: u8,                  // file or directory
    size: u64,                 // in bytes
    permissions: (bool, bool), // (read, write)
    hard_link: Option<Box<Inode>>,
    created_at: Option<u64>,
    updated_at: Option<u64>,
    accessed_at: Option<u64>,
    serial_number: u64,
    data: InodeData,
}

impl Inode {
    pub fn new(mode: u8, name: String, hard_link: Option<Box<Inode>>) -> Inode {
        let serial_number: u64 = unsafe { INODE_SERIAL_NUMER };
        unsafe { INODE_SERIAL_NUMER += 1; }
        if mode == DIR_MODE {
            let size = (size_of::<Inode>() + size_of::<Directory>()) as u64;
            Inode {
                mode,
                size,
                permissions: (true, true),
                hard_link,
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                serial_number,
                data: InodeData::Directory(Directory::new(name)),
            }
        } else {
            let size = (size_of::<Inode>() + size_of::<File>()) as u64;
            Inode {
                mode,
                size,
                permissions: (true, true),
                hard_link,
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                serial_number,
                data: InodeData::File(File::new(name)),
            }
        }
    }

    pub fn new_file_with_data(name: String, data: String, hard_link: Option<Box<Inode>>) -> Inode {
        let size = (size_of::<Inode>() + size_of::<File>() + data.len()) as u64;
        let serial_number: u64 = unsafe { INODE_SERIAL_NUMER };
        unsafe { INODE_SERIAL_NUMER += 1; }
        Inode {
            mode: FILE_MODE,
            size,
            permissions: (true, true),
            hard_link,
            created_at: Some(utils::now_date()),
            updated_at: Some(utils::now_date()),
            accessed_at: Some(utils::now_date()),
            serial_number,
            data: InodeData::File(File::new_with_data(name, data)),
        }
    }

    fn clone(&self) -> Inode {
        Inode {
            mode: self.mode,
            size: self.size,
            permissions: self.permissions,
            hard_link: self.hard_link.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            accessed_at: self.accessed_at,
            serial_number: self.serial_number,
            data: match &self.data {
                InodeData::File(file) => InodeData::File(file.clone()),
                InodeData::Directory(directory) => InodeData::Directory(directory.clone()),
            },
        }
    }

    pub fn get_name(&self) -> &String {
        match &self.data {
            InodeData::File(file) => &file.name,
            InodeData::Directory(directory) => &directory.name,
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn is_file(&self) -> bool {
        self.mode == FILE_MODE
    }

    pub fn is_directory(&self) -> bool {
        self.mode == DIR_MODE
    }

    pub fn print_inode_path(&self, terminal: &mut Stdout) {
        let mut hard_link_tree: Vec<&Inode> = vec![self];
        let mut current_inode = self;
        while let Some(inode) = &current_inode.hard_link {
            hard_link_tree.push(inode);
            current_inode = inode;
        }
        let reversed_tree: Vec<&Inode> = hard_link_tree.into_iter().rev().collect();
        let mut tree_string: String  = String::new();
        for i in 0..reversed_tree.len() {
            if i == 0 {
                tree_string.push_str(reversed_tree[i].get_name());
                continue;
            } else if i == reversed_tree.len() - 1 {
                tree_string.push_str(reversed_tree[i].get_name());
                continue;
            } else {
                tree_string.push_str(reversed_tree[i].get_name());
                tree_string.push_str("/");
            }
        }
        terminal.write(format!("{}>", tree_string).as_bytes()).unwrap();
        terminal.flush().unwrap();
    }

    pub fn add_file(&mut self, file: Inode) {

        if let InodeData::Directory(directory) = &mut self.data {
            self.size += file.size;
            directory.add_file(file);
        } else {
            // todo: handle error
            eprintln!("Error: trying to add a file to a non-directory inode");
        }
    }

    pub fn add_to_size(&mut self, size: u64) {
        self.size += size;
    }

}

#[derive(Debug, Clone)]
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

    pub fn new_with_data(name: String, data: String) -> File {
        File {
            name,
            data,
        }
    }

    pub fn clone(&self) -> File {
        File {
            name: self.name.clone(),
            data: self.data.clone(),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn clone(&self) -> Directory {
        Directory {
            name: self.name.clone(),
            files: self.files.clone(),
        }
    }

    pub fn add_file(&mut self, file: Inode) {
        self.files.push(file);
    }
}
