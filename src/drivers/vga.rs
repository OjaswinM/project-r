// Constants

const VGA_MEM_START: u32 = 0xb8000;
pub const VGA_COLS: usize = 80;
pub const VGA_ROWS: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorCodes {
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
struct VgaColor(u8);

impl VgaColor {
    fn new(fg: ColorCodes, bg: ColorCodes) -> VgaColor {
        VgaColor((bg as u8) << 4 | (fg as u8))
    }
}

#[repr(C)]
struct VgaEntry {
    character: u8,
    color: VgaColor,
}

impl VgaEntry {
    fn new(char: u8, color: VgaColor) -> VgaEntry {
        VgaEntry {
            character: char,
            color: color,
        }
    }
}

#[repr(transparent)]
struct VgaBuffer {
    buf: [[VgaEntry; VGA_COLS]; VGA_ROWS],
}

struct VgaStruct {
    buffer: &'static mut VgaBuffer,
    color: VgaColor,
}

impl VgaStruct {
    pub fn cls(&mut self) -> () {
        for i in 0..(VGA_ROWS) {
            for j in 0..(VGA_COLS) {
                // 32 is the ASCII value for <Space>
                self.buffer.buf[i][j] = VgaEntry::new(b' ', self.color);
            }
        }

        // vga_blink(20, 20);
    }

    pub fn put(&mut self, char: u8, row: usize, col: usize) -> () {
        self.buffer.buf[row][col] = VgaEntry::new(char, self.color)
    }
}

pub fn vga_put(char: u8, row: usize, col: usize) -> () {
    let mut vga = VgaStruct {
        buffer: unsafe { &mut *(VGA_MEM_START as *mut VgaBuffer) },
        color: VgaColor::new(ColorCodes::White, ColorCodes::Black),
    };

    vga.put(char, row, col);
}

pub fn vga_put_yellow(char: u8, row: usize, col: usize) -> () {
    let mut vga = VgaStruct {
        buffer: unsafe { &mut *(VGA_MEM_START as *mut VgaBuffer) },
        color: VgaColor::new(ColorCodes::Yellow, ColorCodes::Black),
    };

    vga.put(char, row, col);
}

pub fn vga_cls() -> () {
    let mut vga = VgaStruct {
        buffer: unsafe { &mut *(VGA_MEM_START as *mut VgaBuffer) },
        color: VgaColor::new(ColorCodes::White, ColorCodes::Black),
    };

    vga.cls()
}

pub fn vga_blink(row: u8, col: u8) {
    use x86_64::instructions::port::PortWriteOnly;

    let index: u16 = (row * VGA_COLS as u8 + col) as u16;

    let mut vga_crt_index: PortWriteOnly<u8> = PortWriteOnly::<u8>::new(0x3d4);
    let mut vga_crt: PortWriteOnly<u8> = PortWriteOnly::<u8>::new(0x3d5);

    unsafe {
        vga_crt_index.write(14 as u8);
        vga_crt.write((index >> 8) as u8);

        vga_crt_index.write(15 as u8);
        vga_crt.write((index) as u8);
    }
}
