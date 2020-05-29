use crate::drivers::vga;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

pub struct TermCursor {
   row: usize,
   col: usize,
}

impl TermCursor {

	fn set_cursor(&mut self, row: usize, col: usize)
	{
		self.row = row;
		self.col = col;
	}


	fn move_cursor(&mut self, steps: usize) -> () {
		self.col += steps;
		
		// wrap to next line if line width increases 80 
		if (self.col / vga::VGA_COLS > 0) {
			self.row += self.col / vga::VGA_COLS;
			self.col = self.col % vga::VGA_COLS;
		}
	}

	pub fn cls(&mut self)
	{
		vga::vga_cls();
		self.set_cursor(0,0);
	}

	fn print(&mut self, s: &str)
	{
		for byte in s.bytes() {
			match byte
			{
				b'\n' => self.set_cursor(self.row + 1, 0),
				b'\r' => self.set_cursor(self.row, 0),
				_ => {
						vga::vga_put(byte, self.row, self.col);
						self.move_cursor(1);
				},
			}
		}
	}
}


impl fmt::Write for TermCursor 
{
    fn write_str(&mut self, s: &str) -> fmt::Result 
	{
        self.print(s);
        Ok(())
    }
}


lazy_static! {
	pub static ref TERM: Mutex<TermCursor> = Mutex::new(TermCursor {
		row: 0,
		col: 0,
	});
}

pub fn TERM_print(s: &str) {
	use core::fmt::Write;
	write!(TERM.lock(), "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
	//TERM.lock().print(s);
}

#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => ($crate::drivers::term::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
        use core::fmt::Write;
		use x86_64::instructions::interrupts;
        //wrtie_fmt() is from Write trait
		interrupts::without_interrupts(|| {
			TERM.lock().write_fmt(args).unwrap();
		});
}



