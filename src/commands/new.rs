use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll, read};
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

fn handle_key_event(event: KeyEvent, inputMode: bool, terminal: &Stdout, mut data: String) -> Option<(bool, bool, String)> {
    /*! Handle the key event and return a tuple with the following values:
    * 1. bool: quit the file editor
    * 2. bool: input mode
    * 3. String: data modified
    */
    if event.kind == KeyEventKind::Press {
        if inputMode {
            match event.code {
                KeyCode::Char(x) => {
                    data.push(x);
                    reload_terminal(&terminal, data.clone());
                    Some((false, inputMode, data))
                },
                KeyCode::Backspace => {
                    data.pop();
                    reload_terminal(&terminal, data.clone());
                    Some((false, inputMode, data))
                },
                KeyCode::Enter => {
                    data.push('\n');
                    reload_terminal(&terminal, data.clone());
                    Some((false, inputMode, data))
                },
                KeyCode::Esc => {
                    Some((false, false, data))
                },
                _ => None
            }
        } else {
            match event.code {
                KeyCode::Char(x) => {
                    if x == 's' && event.modifiers == KeyModifiers::CONTROL {
                        return Some((true, inputMode, data));
                    } else if x == 'i' {
                        return Some((false, true, data));
                    } else {
                        None
                    }
                },
                _ => None
            }
        }
    } else{
        None
    }
}

pub fn create_new_file(name: String, hard_link: Inode) -> Inode {
    let mut terminal: Stdout = stdout();
    let mut quit: bool = false;
    let mut data: String = String::new();
    let mut input_mode: bool = false;
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
                    match handle_key_event(event, input_mode, &terminal, data.clone()) {
                        Some((q, im, d)) => {
                            quit = q;
                            input_mode = im;
                            data = d;
                        },
                        None => {}
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