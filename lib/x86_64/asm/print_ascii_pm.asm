[bits 32]		; protected mode

VIDEO_MEMORY equ 0xb8000		; start of video controller's memory area
WHITE_ON_BLACK equ 0x0f		; black bg on white fg

print_ascii_pm:
; prints the string in 32 bit protected mode. The string is stored directly to
; the Video Controller's memory which usually starts at 0xb8000. This memory is
; arranged as an array of 16 bit words where each word describes one pixel of 
; the 80x25 screen. The lower 8 bits of each word contain the ASCII character
; to print at that pixel and the upper 8 bits give the foreground and backgro
; -und color of the pixel.
; ~~~~~~~~~~~~~~~ REGISTERS ~~~~~~~~~~~~~~~
; edx: 32 bit address of the video memory ie. 0xb800
; ebx: address of the null terminated string
; al: ASCII value of the character pointed by ebx
; ah: the bg and fg color of the pixel
	pusha
	mov edx, VIDEO_MEMORY		

print_string_pm_loop:
	mov al, [ebx]		
	mov ah, WHITE_ON_BLACK

	cmp al, 0
	je print_string_pm_exit

	mov [edx], ax

	add ebx, 1
	add edx, 2

	jmp print_string_pm_loop

print_string_pm_exit:
	popa
	ret

