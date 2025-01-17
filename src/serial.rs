use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! { //to ensure the init method is called exactly once on its first use
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts; //import x86_64 interrupt handling utilities

    interrupts::without_interrupts(|| { // without_interrupts function takes a closure and executes it in an interrupt-free environment
        SERIAL1
            .lock() //no interrupts as long as the Mutex is locked
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

///prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print{
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

///prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println{
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}