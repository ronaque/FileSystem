use std::io::{Stdout, Write};
use super::utils;
pub const DIR_MODE: u8 = 0;
pub const FILE_MODE: u8 = 1;

const ROOT_INODE: u64 = 0;
static mut INODE_SERIAL_NUMER: u64 = 0;

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
            Inode {
                mode,
                size: 0,
                permissions: (true, true),
                hard_link,
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                serial_number,
                data: InodeData::File(File::new(name)),
            }
        } else {
            Inode {
                mode,
                size: 0,
                permissions: (true, true),
                hard_link,
                created_at: Some(utils::now_date()),
                updated_at: Some(utils::now_date()),
                accessed_at: Some(utils::now_date()),
                serial_number,
                data: InodeData::Directory(Directory::new(name)),
            }
        }
    }

    pub fn get_name(&self) -> &String {
        match &self.data {
            InodeData::File(file) => &file.name,
            InodeData::Directory(directory) => &directory.name,
        }
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
