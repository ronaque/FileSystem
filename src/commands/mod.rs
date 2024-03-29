use crate::types::Inode;
mod new_file;

pub fn create_new_file(name: String, hard_link: &mut Inode) -> Result<(), &'static str> {
    new_file::create_new_file(name, hard_link)
}