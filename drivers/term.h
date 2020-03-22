#ifndef _TERM_H
#define _TERM_H

#include "vga.h"

/* ~~~~~~~~~~~ Constants ~~~~~~~~~~ */

// indicates where the next character will be printed
struct cursor {
	// the current row being pointed to by the cursor
	uint8_t x;
	
	// the current col  being pointed to by the cursor
	uint8_t y;

	// pointer to the next address the cursor will write to
	uint16_t* curp;
};

// parse the string for escape characters and print the final string on 
// screen
void term_print(char* str);

// clear the screen and reset the cursor to 0th position
void term_clear();

// move the cursor by specified steps, negative offsets are also accepted
void term_cursor_move(uint16_t steps);

// set the cursor to the specified x and y value
void term_cursor_set(uint8_t x, uint8_t y);

#endif

