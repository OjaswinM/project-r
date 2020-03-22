#include "vga.h"

void vga_put(char ch, uint16_t* addr)
{
	// print the character in black & white at addr
	*(addr) = (uint16_t)(ch | BLACK_N_WHITE << 8);
	return;
}

void vga_cls()
{
	uint16_t* ptr = (uint16_t *)VGA_MEM_START;
	for (uint16_t i = 0; i < (VGA_X * VGA_Y); i++) {
		*(ptr) = (uint16_t)0x0;	
		ptr++;
	}
	return;
}
