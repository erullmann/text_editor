use termios::*;
use std::error::Error;
use filedescriptor::*;

pub struct RawMode {
    fd: RawFileDescriptor,
    original_termios: Termios
}

impl RawMode {
    pub fn new(new_fd: RawFileDescriptor) -> Result<RawMode, Box<dyn Error>>{
        let raw_mode = RawMode {
            fd: new_fd,
            original_termios: Termios::from_fd(new_fd)?
        };
        raw_mode.enable_raw_mode()?;
        return Ok(raw_mode);
    }

    fn disable_raw_mode(&self) -> Result<(), Box<(dyn Error + 'static)>> {
        tcsetattr(self.fd, TCSAFLUSH, &self.original_termios)?;
        return Ok(());
    }

    fn enable_raw_mode(&self) -> Result<(), Box<(dyn Error + 'static)>> {
        let mut termios = Termios::from_fd(self.fd)?;
        cfmakeraw(&mut termios);
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 1;
        tcsetattr(self.fd, TCSAFLUSH, &termios)?;
        return Ok(());
    }
}

impl Drop for RawMode {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.disable_raw_mode();
    }
}
