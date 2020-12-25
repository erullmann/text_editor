
use std::io::{self, Read};
use filedescriptor::*;
use text_editor::*;
mod raw_mode;


fn main() -> io::Result<()> {
    let _raw_mode = raw_mode::RawMode::new(io::stdin().as_raw_file_descriptor());
    if _raw_mode.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "Could not use raw mode"));
    }

    loop {
        let mut buffer = [0; 1];
        let char_count = io::stdin().read(&mut buffer)?;
        
        let c = if char_count == 1 {
            buffer[0] as char
        }
        else {
            '\0'
        };
        
        if buffer[0].is_ascii_control() {
            print!("{}\r\n", buffer[0])
        }
        else {
            print!("{}, ({})\r\n", buffer[0], c);
        }
        if c == ctrl!('q') {
            break;
        }
    }
    return Ok(());
}
