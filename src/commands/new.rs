use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll, read};
use crossterm::{QueueableCommand, terminal};
use crossterm::cursor::{MoveTo, MoveLeft, MoveUp, position};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::types::{FILE_MODE, Inode};

struct GapBuffer {
    data: Vec<Vec<char>>,
    line_index: u16,
    col_index: u16,
}

impl GapBuffer{
    fn new() -> GapBuffer {
        GapBuffer {
            data: vec![vec![]],
            line_index: 0,
            col_index: 0,
        }
    }

    fn clone(&self) -> GapBuffer {
        GapBuffer {
            data: self.data.clone(),
            line_index: self.line_index,
            col_index: self.col_index,
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut idx_line: u16 = 1;
        for line in &self.data {
            for c in line {
                result.push(*c);
            }
            if idx_line < self.data.len() as u16 {
                result.push('\n');
            }
            idx_line += 1;
        }
        result
    }

    fn push(&mut self, c: char) {
        self.data[self.line_index as usize].insert(self.col_index as usize, c);
        self.col_index += 1;
    }

    fn push_line(&mut self) {
        self.data.insert(self.line_index as usize + 1, vec![]);
        self.line_index += 1;
        self.col_index = 0;
    }


    fn remove(&mut self) {
        if self.col_index > 0 {
            self.col_index -= 1;
            self.data[self.line_index as usize].remove(self.col_index as usize);
        } else if self.line_index > 0 {
            self.line_index -= 1;
            self.data.remove(self.line_index as usize);
        }
    }

    fn move_left(&mut self) {
        // This function will move the cursor to the left
        // If it is in the first column, it will move to the previous line
        if self.col_index == 0 {
            // Move to Previous Line
            if self.line_index > 0 {
                self.line_index -= 1;
                self.col_index = (self.data[self.line_index as usize].len()) as u16;
            }
        } else if self.col_index > 1 {
            // Move to the previous character
            self.col_index -= 1;
        } else {
            // Move to the beginning of the line
            self.col_index = 0;
        }
    }

    fn move_right(&mut self) {
        // This function will move the cursor to the right
        // If it is in the last column, it will move to the next line
        if self.col_index == self.data[self.line_index as usize].len() as u16 {
            // Move to Next Line
            if self.line_index + 1 < self.data.len() as u16 {
                self.line_index += 1;
                self.col_index = 0;
            }
        } else if self.col_index + 1 < self.data[self.line_index as usize].len() as u16 {
            // Move to the next character
            self.col_index += 1;
        } else {
            // Move to the front of the last character in the line
            self.col_index = self.data[self.line_index as usize].len() as u16;
        }
    }

    fn move_up(&mut self) {
        // This function will move the cursor up if it isn't in the first line
        if self.line_index > 0 {
            if self.data[(self.line_index - 1) as usize].len() > self.col_index as usize {
                self.line_index -= 1;
            } else {
                self.line_index -= 1;
                self.col_index = self.data[self.line_index as usize].len() as u16;
            }
        }
    }

    fn move_down(&mut self) {
        // This function will move the cursor down if it isn't in the last line
        if self.line_index + 1 < self.data.len() as u16 {
            if self.data[(self.line_index + 1) as usize].len() > self.col_index as usize {
                self.line_index += 1;
            } else {
                self.line_index += 1;
                self.col_index = self.data[self.line_index as usize].len() as u16;
            }
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
fn reload_terminal_input_mode(mut terminal: &Stdout, data: GapBuffer) {
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0, 0)).unwrap();
    terminal.write(data.to_string().as_bytes()).unwrap();
    terminal.queue(MoveTo(data.col_index, data.line_index)).unwrap();
    terminal.flush().unwrap();
}

fn handle_key_event(event: KeyEvent, input_mode: &mut bool, quit: &mut bool, terminal: &Stdout, mut data: GapBuffer) -> GapBuffer {
    // Handle the key event, and return the possible modified data
    if event.kind == KeyEventKind::Press {
        if *input_mode {
            match event.code {
                KeyCode::Char(x) => {
                    data.push(x);
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Left => {
                    data.move_left();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Right => {
                    data.move_right();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Up => {
                    data.move_up();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Down => {
                    data.move_down();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Backspace => {
                    data.remove();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Enter => {
                    data.push_line();
                    reload_terminal_input_mode(&terminal, data.clone());
                    *quit = false;
                    data
                },
                KeyCode::Esc => {
                    *input_mode = false;
                    *quit = false;
                    data
                },
                _ => data
            }
        } else {
            match event.code {
                KeyCode::Char(x) => {
                    if x == 's' && event.modifiers == KeyModifiers::CONTROL {
                        *quit = true;
                        data
                    } else if x == 'i' {
                        reload_terminal_input_mode(&terminal, data.clone());
                        *input_mode = true;
                        *quit = false;
                        data
                    } else {
                        data
                    }
                },
                _ => data
            }
        }
    } else{
        data
    }
}

fn create_gap_buffer() -> String {
    let mut terminal: Stdout = stdout();
    let mut quit: bool = false;
    let mut data: GapBuffer = GapBuffer::new();
    let mut input_mode: bool = false;

    EnterAlternateScreen;
    enable_raw_mode().expect("Raw Mode of terminal not enabled");
    terminal.queue(MoveTo(0, 0)).unwrap();
    reload_terminal_command_mode(&terminal, data.to_string().as_str());
    let (mut w, mut h) = terminal::size().unwrap();

    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                },
                Event::Key(event) => {
                    data = handle_key_event(event, &mut input_mode, &mut quit, &terminal, data.clone());
                    if !input_mode {
                        reload_terminal_command_mode(&terminal, data.to_string().as_str());
                    }
                    if quit {
                        break;
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

    data.to_string()
}

pub fn create_new_file(name: String, hard_link: Inode) -> Inode {
    let file_data: String = create_gap_buffer();

    return Inode::new(FILE_MODE, name, Some(Box::new(hard_link)));
    // Todo!("Create the inode with the file data and name");
    // Todo!("Add the new file to the current directory");
    // Todo!("Calculate the file size and store on the inode, and directory size recursively");
}