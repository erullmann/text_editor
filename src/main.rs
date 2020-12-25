use std::io::{self, Read, Write};
use filedescriptor::*;
use text_editor::*;
mod raw_mode;

const CTRL_Q: char = ctrl!('q');

fn read_key() -> io::Result<Option<char>> {
    let mut buffer = [0; 1];
    let char_count = io::stdin().read(&mut buffer)?;

    Ok(if char_count == 1 {
        Some(buffer[0] as char)
    }
    else {
        None
    })
}

fn process_keypress(raw_mode: &raw_mode::RawMode) -> io::Result<()> {
    let c = read_key()?;
    
    match c {
        Some(CTRL_Q) => {
            raw_mode.disable_raw_mode();
            std::process::exit(0);
        },
        Some(_) => {}, // do nothing
        None => {} // do nothing
    };
    Ok(())
}

fn refresh_screen() -> io::Result<()> {
    io::stdout().write("\x1b[2J".as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let raw_mode = raw_mode::RawMode::new(io::stdin().as_raw_file_descriptor());
    if raw_mode.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "Could not use raw mode"));
    }
    let raw_mode = raw_mode.unwrap();
    refresh_screen()?;
    loop {
        refresh_screen()?;
        process_keypress(&raw_mode)?;
    }
}
