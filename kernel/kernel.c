 #include "vga.h"

void main() {
	char* vid = (char *)0xb8000;
	*(vid) = 'X';
	// Display an X on screen
	uint16_t X = 0xF0 << 8 | 'X'; 
	vga_put(X);
}
