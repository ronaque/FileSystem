use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, poll, read};
use crossterm::{execute, QueueableCommand, terminal};
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::types::{FILE_MODE, Inode};


fn reload_terminal(mut terminal: &Stdout, data: String) {
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    terminal.write(data.as_bytes()).unwrap();
    terminal.flush().unwrap();
}
pub fn create_new_file(name: String, hard_link: Inode) -> Inode {
    let mut terminal: Stdout = stdout();
    let mut quit: bool = false;
    let mut data: String = String::new();
    EnterAlternateScreen;
    enable_raw_mode();
    terminal.write(b"Alternate Screen in raw mode").unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    reload_terminal(&terminal, data.clone());
    let (mut w, mut h) = terminal::size().unwrap();
    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                },
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char(x) => {
                                if x == 'c' && event.modifiers == KeyModifiers::CONTROL {
                                    quit = true;
                                } else {
                                    data.push(x);
                                    reload_terminal(&terminal, data.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    quit = true;
                }
            }
        }
        sleep(Duration::from_millis(33));
    }

    LeaveAlternateScreen;
    disable_raw_mode();
    println!("");

    return Inode::new(FILE_MODE, name, Some(Box::new(hard_link)));
    // Todo!("Make a CLI, vim-like, to write the content of the file");
    // Todo!("Create the inode with the file data and name");
    // Todo!("Add the new file to the current directory");
    // Todo!("Calculate the file size and store on the inode, and directory size recursively");
}