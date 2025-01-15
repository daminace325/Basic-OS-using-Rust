use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color { //using enum to represent different colours
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8); 

impl ColorCode { //a full color code that specifies foreground and background color
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//define VGA text buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


// ensures to make VGA buffer volatile
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


// create a writer type to write to screen
pub struct Writer {
    column_position: usize, //keeps track of the current position in the last row
    color_code: ColorCode, //current foreground and background colors
    buffer: &'static mut Buffer, //reference to the VGA buffer is stored and static specifies that the reference is valid for the whole program run time 
}


//create a method to write a single ASCII byte
impl Writer { 
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), //if \n encountered in call new_line() method
            byte => { //other bytes get printed on screen
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar { //update the write_byte method with new ScreenChar
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

     //implementation of new line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    //clears a row by overwriting all of its characters with a space character
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    //to print whole strings convert them to bytes and print them one-by-one
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}


//using Rust's formatting macros to print different types, like integers or floats
impl fmt::Write for Writer { 
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! { //since one-time initialization of statics with non-const functions is a common problem in Rust hence use lazy_static
    //spinning mutex to add safe interior mutability to static WRITER
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


//use the println! and print! macros, but modify them to use as _print function
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}