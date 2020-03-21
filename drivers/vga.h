#ifndef _VGA_H
#define _VGA_H

#include <stdint.h>

// ~~~~~~~~~~~~ VGA ~~~~~~~~~~~~~
#define VGA_MEM_START 0xb8000
#define VGA_X 80
#define VGA_Y 25

// ~~~~~~~~~~~~ COLORS ~~~~~~~~~~~~~
#define BLACK_N_WHITE 0x0f

// pointer to the current address
struct cursor {
	// the current row being pointed to by the cursor
	uint8_t x;
	
	// the current col  being pointed to by the cursor
	uint8_t y;

	// pointer to the next address the cursor will write to
	uint16_t* curp;
};

// encode bg and fg color into 8 bit values to display on the screen
inline uint8_t vga_color(uint8_t fg, uint8_t bg)
{
	return fg | bg << 4;
}

// encode color and character to display on the screen
inline uint8_t vga_entry(uint8_t color, uint8_t ch)
{
	return ch | color << 8; 
}

// convert (row, col) to corresponding offset in VGA memory array
inline uint16_t vga_offset(uint8_t x, uint8_t y)
{
	return (x + y * 80);	
}

// move the cursor by specified steps, negative offsets are also accepted
void vga_cursor_move(uint16_t steps);

// set the cursor to the specified x and y value
void vga_cursor_set(uint8_t x, uint8_t y);

// enter a character at given cursor position. The characcter is in black and white
void vga_put(uint16_t entry);

// enter a string at given cursor position.
void vga_puts(uint16_t entry);

// fill the whole screen with black background
void vga_cls();
#endif
