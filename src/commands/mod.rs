use crate::types::Inode;
mod new_file;

pub fn create_new_file(name: String, parent_inode: &mut Inode) -> Result<(), &'static str> {
    new_file::create_new_file(name, parent_inode)
}