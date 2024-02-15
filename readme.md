# Filesystem

This repo implements a "virtual" CLI filesystem that i have conceptualised based on [osdev](https://wiki.osdev.org/File_Systems) wiki explaining FileSystems, and directory/file commands of linux and windows CMDs.

This was created just for fun!

### The objectives to be implemented are:
- [ ] Tracking the available storage space
- [ ] Tracking which block or blocks of data belong to which files
- [ ] Creating new files
- [ ] Reading data from existing files into memory
- [ ] Updating the data in the files
- [ ] Deleting existing files

#### Additionally
- [ ] Assigning human-readable names to files, and renaming files after creation
- [ ] Allowing files to be divided among non-contiguous blocks in storage, and tracking the parts of files even when they are fragmented across the medium
- [ ] Providing some form of hierarchical structure, allowing the files to be divided into directories or folders
- [ ] Buffering reading and writing to reduce the number of actual operation on the physical medium
- [ ] Caching frequently accessed files or parts of files to speed up access
- [ ] Allowing files to be marked as 'read-only' to prevent unintentional corruption of critical data
- [ ] Providing a mechanism for preventing unauthorized access to a user's files

It will be implemented based on the inode indexing method as reference

Ref:
- https://www.youtube.com/watch?v=6KjMlm8hhFA
- https://www.youtube.com/watch?v=tMVj22EWg6A
- https://blog.carlosgaldino.com/writing-a-file-system-from-scratch-in-rust.html


## DevBlog

#### 15/02/24 
Started implementing the code, learning about the basics of rust, and the escape ANSI code to manipulate the terminal.