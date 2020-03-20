#include "vga.h"
 
void main() {
	// Display an X on screen
	uint16_t X = 0x0F << 8 | 'X'; 
	for (int i = 0; i < (25*80 - 1); i++) {
		vga_put(X);
	}
}
