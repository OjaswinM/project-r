global long_mode_start

[bits 64]
section .text
long_mode_start:
	cli
	
	mov rax, 0x2f592f412f4b2f4f
 	mov [0xb8000], rax
	extern _start
	call _start

.inf:
	hlt
	jmp .inf
