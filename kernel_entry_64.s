[bits 64]

section .text
global long_mode_start
long_mode_start:
	cli
	mov rax, 0x2f592f412f4b2f4f
 	mov [0xb8000], rax

.inf:
	hlt
	jmp .inf
