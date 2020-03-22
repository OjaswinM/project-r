#ifndef _VGA_H
#define _VGA_H

#include <stdint.h>

/* ~~~~~~~~~~~~ VGA ~~~~~~~~~~~~~ */
#define VGA_MEM_START 0xb8000
#define VGA_X 80
#define VGA_Y 25

/* ~~~~~~~~~~~~ COLORS ~~~~~~~~~~~~~ */
#define BLACK_N_WHITE 0x0f

/* ~~~~~~~~~~~~ MACROS ~~~~~~~~~~~~~ */

#define VGA_OFFSET(X, Y) (X + Y * 80)

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

// enter a character at the given address addr. The character is in black and white
void vga_put(char ch, uint16_t* addr);

// fill the whole screen with black background
void vga_cls();
#endif
