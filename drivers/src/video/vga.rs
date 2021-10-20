use core::fmt::Write;

#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[repr(packed)]
struct Char {
    pub char: u8,
    attr: u8
}

impl Char {
    #[inline]
    pub fn new(c: char, color: Color) -> Char {
        Self {
            char: c as u8,
            attr: color as u8,
        }
    }
}

pub const SCREEN_HEIGHT: usize = 25;
pub const SCREEN_WIDTH: usize = 25;

pub const SCREEN_SIZE: usize = SCREEN_WIDTH*SCREEN_HEIGHT;
type Screen = [Char; SCREEN_SIZE];
pub static SCREEN: *mut Screen = 0xb8000 as *mut Screen;

pub fn clear_screen(bg: Color) {
    unsafe {

    }
}