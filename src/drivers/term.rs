use crate::drivers::vga;
use lazy_static::lazy_static;
use spin::Mutex;

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

	fn cls(&mut self)
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

lazy_static! {
	pub static ref term: Mutex<TermCursor> = Mutex::new(TermCursor {
		row: 0,
		col: 0,
	});
}

pub fn term_print(s: &str) {
	term.lock().print(s);
}






