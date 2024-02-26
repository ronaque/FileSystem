use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll, read};
use crossterm::{QueueableCommand, terminal};
use crossterm::cursor::{MoveTo, MoveLeft};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::types::{FILE_MODE, Inode};

struct GapBuffer {
    data: String,
    index: u16,
}

impl GapBuffer{
    fn new() -> GapBuffer {
        GapBuffer {
            data: String::new(),
            index: 0,
        }
    }

    fn clone(&self) -> GapBuffer {
        GapBuffer {
            data: self.data.clone(),
            index: self.index,
        }
    }

    fn push(&mut self, c: char) {
        self.data.insert(self.index as usize, c);
        self.index += 1;
    }

    fn remove(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.data.remove(self.index as usize);
        }
    }

    fn move_left(&mut self, n: u16) {
        if self.index > n {
            self.index -= n;
        } else {
            self.index = 0;
        }
    }

    fn move_right(&mut self, n: u16) {
        if self.index + n < self.data.len() as u16 {
            self.index += n;
        } else {
            self.index = self.data.len() as u16;
        }
    }
}

fn reload_terminal_command_mode(mut terminal: &Stdout, data: &str) {
    let (w, h) = terminal::size().unwrap();
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    terminal.write(data.as_bytes()).unwrap();
    terminal.queue(MoveTo(0, h-2)).unwrap();
    let bar = "─".repeat(w as usize);
    terminal.write(bar.as_bytes()).unwrap();
    terminal.queue(MoveTo(0, h-1)).unwrap();
    terminal.write(b"Ctrl+S: Save | I: Insert mode").unwrap();
    terminal.flush().unwrap();
}
fn reload_terminal_input_mode(mut terminal: &Stdout, data: &str, horizontal_moves: u16) {
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    terminal.write(data.as_bytes()).unwrap();
    if horizontal_moves > 0 {
        terminal.queue(MoveLeft(horizontal_moves)).unwrap();
    }
    terminal.flush().unwrap();
}

fn handle_key_event(event: KeyEvent, input_mode: bool, terminal: &Stdout, mut data: GapBuffer, horizontal_moves: &mut usize) -> Option<(bool, bool, GapBuffer)> {
    /*! Handle the key event and return a tuple with the following values:
    * 1. bool: quit the file editor
    * 2. bool: input mode
    * 3. String: data modified
    */
    if event.kind == KeyEventKind::Press {
        if input_mode {
            match event.code {
                KeyCode::Char(x) => {
                    data.push(x);
                    reload_terminal_input_mode(&terminal, data.data.as_str(), horizontal_moves.clone() as u16);
                    Some((false, input_mode, data))
                },
                KeyCode::Left => {
                    *horizontal_moves += 1;
                    data.move_left(1);
                    reload_terminal_input_mode(&terminal, data.data.as_str(), horizontal_moves.clone() as u16);
                    Some((false, input_mode, data))
                },
                KeyCode::Backspace => {
                    data.remove();
                    reload_terminal_input_mode(&terminal, data.data.as_str(), horizontal_moves.clone() as u16);
                    Some((false, input_mode, data))
                },
                KeyCode::Enter => {
                    reload_terminal_input_mode(&terminal, data.data.as_str(), horizontal_moves.clone() as u16);
                    data.push('\n');
                    Some((false, input_mode, data))
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
                        return Some((true, input_mode, data));
                    } else if x == 'i' {
                        reload_terminal_input_mode(&terminal, data.data.as_str(), horizontal_moves.clone() as u16);
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
    let mut data: GapBuffer = GapBuffer::new();
    let mut input_mode: bool = false;
    let mut horizontal_moves = 0;
    EnterAlternateScreen;
    enable_raw_mode().expect("Raw Mode of terminal not enabled");
    terminal.queue(MoveTo(0, 0)).unwrap();
    reload_terminal_command_mode(&terminal, data.data.as_str());
    let (mut w, mut h) = terminal::size().unwrap();
    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                },
                Event::Key(event) => {
                    match handle_key_event(event, input_mode, &terminal, data.clone(), &mut horizontal_moves) {
                        Some((q, im, d)) => {
                            if !im {
                                reload_terminal_command_mode(&terminal, d.data.as_str());
                            }
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
    disable_raw_mode().expect("Exit raw mode of terminal failed");
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    terminal.flush().unwrap();

    return Inode::new(FILE_MODE, name, Some(Box::new(hard_link)));
    // todo!("Make a CLI, vim-like, to write the content of the file");
    // Todo!("Create the inode with the file data and name");
    // Todo!("Add the new file to the current directory");
    // Todo!("Calculate the file size and store on the inode, and directory size recursively");
}