#include "vga.h"

static struct cursor cur = {0, 0, (uint16_t *)VGA_MEM_START};

void vga_put(uint16_t entry)
{
	*(cur.curp) = entry;
	vga_cursor_move(1);
	return;
}

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
