void mail() {
	// create a pointer to the video memory area
	char* vid_addr = (char *)0xb8000;
	// Display an X at that address
	*vid_addr = 'B';
}
