#include "term.h"

static struct cursor cur = {0, 0, (uint16_t *)VGA_MEM_START};

void term_cursor_move(uint16_t steps)
{
	cur.curp += steps;
	cur.x += steps;
	
	// wrap to next line if line width increases 80 
	if (cur.x / VGA_X > 0) {
		cur.x = cur.x % VGA_X;
		cur.y += cur.x / VGA_X;
	}
	return;
}

void term_cursor_set(uint8_t x, uint8_t y)
{
	cur.x = x;
	cur.y = y;
	cur.curp = ((uint16_t*)VGA_MEM_START) + VGA_OFFSET(x, y);
	return;
}

void term_clear()
{
	vga_cls();
	term_cursor_set(0, 0);
}

void term_print(char* str)
{
	char* ch = str;
	while (*ch) {
		switch (*ch) {
			case '\n':
				// set the cursor to beginning of next line
				term_cursor_set(0, cur.y + 1);
				break;
			case '\r':
				// set the cursor to beginning of current line
				term_cursor_set(0, cur.y);
				break;
			default:
				// not an escape character
				vga_put(*ch, cur.curp);
				term_cursor_move(1);
				break;
		}	
		ch++;
	}

	return;
}
