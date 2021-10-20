use core::fmt;

use crate::video::vga;

struct Stdout;

impl Stdout {
    fn write_fmt(&mut self, fmt: fmt::Arguments) {
        fmt::write(self, fmt);
    }
}

impl fmt::Write for Stdout {
    fn write_char(&mut self, c: char) -> fmt::Result {
        unsafe {

        }
    }
}