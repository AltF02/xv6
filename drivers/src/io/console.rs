use core::fmt;

use crate::video::vga;

struct Console;

impl Console {
    pub unsafe fn put_char(&self) {}
}

impl fmt::Write for Console {
    fn write_str(&mut self, c: &str) -> fmt::Result {
        unsafe {
            self.put_char();
        }
        write!()
    }
}
