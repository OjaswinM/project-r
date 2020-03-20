#ifndef _VGA_H
#define _VGA_H

#include <stdint.h>

#define VGA_MEM_START 0xb8000
#define VGA_COLS 80
#define VGA_ROWS 25

// pointer to the current address
struct cursor {
	unsigned x;
	unsigned y;
	uint16_t* curp;
};

// encodes bg and fg color into 8 bit values to display on the screen
inline uint8_t vga_color(uint8_t fg, uint8_t bg)
{
	return fg | bg << 4;
}

// enter a value to given cursor position
void vga_put(uint16_t entry);

// convert (row, col) to corresponding offset in VGA memory array
inline int vga_offset(int x, int y)
{
	return (x * 80 + y);	
}

// function to move the cursor by specified steps, negative offsets are also accepted
void vga_cursor_move(int steps);


#endif
