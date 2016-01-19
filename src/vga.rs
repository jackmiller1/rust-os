use core::ptr::write_bytes;
use core::ptr::Unique;
use spin::Mutex;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;
const TEXT_BUFFER: usize = 0xb8000;

// struct Size {
//     width: usize,
//     height: usize
// }

// struct Vga {
//     size: Size,
//     buffer: usize,
//     // Add Ports
// }



pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    cursor: Cursor{ row: 0, col: 0 },
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(TEXT_BUFFER as *mut _) }
});

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
          self.write_character(byte)
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15
}

pub fn clear_screen() {
    unsafe {
        write_bytes(TEXT_BUFFER as *mut u8, 0x00, SCREEN_WIDTH*SCREEN_HEIGHT);
    }
}

struct Cursor {
    row: usize,
    col: usize
}

impl Cursor {

    // pub fn update_vga(&self) {
    //     let position: usize = (self.row*SCREEN_WIDTH) + self.col;
    //     write_port(0x3D4, 0x0F);
    //     write_port(0x3D5, (position & 0xFF) as u8);
    //     write_port(0x3D4, 0x0E);
    //     write_port(0x3D5, ((position >> 8) & 0xFF) as u8);
    // }
}

#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    character: u8,
    color: ColorCode,
}


struct TextBuffer {
    chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

pub struct Writer {
    cursor: Cursor,
    buffer: Unique<TextBuffer>,
    color_code: ColorCode
}

impl Writer {
    pub fn write_character(&mut self, ascii_char: u8) {
        match ascii_char {
            b'\n' => self.new_line(),
            ascii_char => {
                if self.cursor.col >= SCREEN_WIDTH {
                    self.new_line();
                }

                self.buffer().chars[self.cursor.row][self.cursor.col] = ScreenChar {
                    character: ascii_char,
                    color: self.color_code
                };
                self.cursor.col += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut TextBuffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        self.cursor.col = 0;
        self.cursor.row += 1;
    }
    
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            color: self.color_code
        };
        self.buffer().chars[row] = [blank; SCREEN_WIDTH];
    }
}
