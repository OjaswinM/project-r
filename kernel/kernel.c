//#include "vga.h"
#include <stdint.h>

#define VGA_MEM_START 0xb8000
#define VGA_COLS 80
#define VGA_ROWS 25

struct cursor {
	unsigned x;
	unsigned y;
	uint16_t* curp;
};

static struct cursor cur = {0, 0, (uint16_t *)VGA_MEM_START};

void vga_cursor_move(int steps)
{
	cur.curp += steps;

	cur.x += steps;
	// wrap to next line if line width increases 80
	while (cur.x % (VGA_COLS - 1) > 0) {
		cur.x = cur.x - VGA_COLS;
		cur.y++;
	}
	return;
}

void vga_put(uint16_t entry)
{
	*(cur.curp) = entry;
	vga_cursor_move(1);
	return;
}

void main() {
	// Display an X on screen
	uint16_t X = 0xF0 << 8 | 'X'; 
	vga_put(X);
}
