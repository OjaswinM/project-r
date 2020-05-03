// Constants

const VGA_MEM_START: u32 = 0xb8000;
const VGA_COLS: usize = 80;
const VGA_ROWS: usize = 24;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum color_codes {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    Lt_gray = 7,
    Dark_gray = 8,
    Lt_blue = 9,
    Lt_green = 10,
    Lt_cyan = 11,
    Lt_red = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct vga_color(u8);

impl vga_color {
	fn new(fg: color_codes, bg: color_codes) -> vga_color {
		vga_color((bg as u8) << 4 | (fg as u8))
	}
}

#[repr(C)]
struct vga_entry {
	character: u8,
	color: vga_color,
}

impl vga_entry {
	fn new(char: u8, color: vga_color) -> vga_entry {
		vga_entry { 
			character: char,
			color: color,
		}		
	}
}

#[repr(transparent)]
struct vga_buffer {
	buf: [[vga_entry; VGA_COLS]; VGA_ROWS],
}

struct vga_struct {
	buffer: &'static mut vga_buffer,
	color: vga_color,
}

impl vga_struct {
	
	pub fn cls(&mut self) -> () {
		for i in 0..VGA_ROWS  {
			for j in 0..VGA_COLS {
				// 32 is the ASCII value for <Space>
				self.buffer.buf[i][j] = vga_entry::new(32, self.color);
			}
		}
	}

	pub fn put(&mut self, char: u8, row: usize, col: usize) -> () {
		self.buffer.buf[row][col] = vga_entry::new(char, self.color)
	}
}


pub fn vga_put(char: u8, row: usize, col: usize) -> () {
	let mut vga = vga_struct {
		buffer: unsafe { &mut *(VGA_MEM_START as *mut vga_buffer) },
		color: vga_color::new(color_codes::White, color_codes::Black),
	};

	vga.put(char, row, col);
}







