use crate::types::Inode;
mod new;

pub fn create_new_file(name: String, hard_link: Inode) -> Inode {
    new::create_new_file(name, hard_link)
}