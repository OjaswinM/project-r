#include "vga.h"

static struct cursor cur = {0, 0, (uint16_t *)VGA_MEM_START};

void vga_cursor_move(uint16_t steps)
{
	cur.curp += steps;
	cur.x += steps;
	
	// wrap to next line if line width increases 80 
	// while (cur.x % (VGA_X - 1) > 0) {
	// 	cur.x = cur.x - VGA_X;
	// 	cur.y++;
	// }
	if (cur.x / VGA_X > 0) {
		cur.x = cur.x % VGA_X;
		cur.y += cur.x / VGA_X;
	}
	return;
}

void vga_cursor_set(uint8_t x, uint8_t y)
{
	cur.x = x;
	cur.y = y;
	cur.curp = ((uint16_t*)VGA_MEM_START) + OFFSET(x, y);
	return;
}

void vga_put(char ch)
{
	// print the character in black & white
	*(cur.curp) = (uint16_t)(ch | BLACK_N_WHITE << 8);
	vga_cursor_move(1);
	return;
}

void vga_puts(char* string)
{
	char* ch = string;
	while (*ch) {
		vga_put(*ch);
		ch++;
	}
	return;
}

void vga_cls()
{
	uint16_t* ptr = (uint16_t *)VGA_MEM_START;
	for (uint16_t i = 0; i < (VGA_X * VGA_Y); i++) {
		*(ptr) = (uint16_t)0x0;	
		ptr++;
	}
	vga_cursor_set(0, 0);
	return;
}


