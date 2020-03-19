; a bootloader that spans 1 sector(512 bytes). It directly switches to 32 bit 
; protected mode
[bits 16]
[org 0x7c00]

KERN_OFFSET equ 0x1000		; address where kernel will be loaded
SECTORS_TO_LOAD equ 1

	mov [BOOT_DRIVE], dl

	mov bp, 0x9000
	mov sp, bp

	mov bx, MSG_REAL_MODE
	call print_ascii

	call load_kernel
		
	call switch_to_pm

	jmp $

%include "print_ascii.asm"
%include "gdt.asm"
%include "switch_to_pm.asm"
%include "print_ascii_pm.asm"
%include "disk_load.asm"

[bits 16] 

load_kernel:
	mov bx, MSG_LOAD_KERN
	call print_ascii

	mov bx, KERN_OFFSET
	mov dh, SECTORS_TO_LOAD
	mov dl, [BOOT_DRIVE]
	call disk_load
	
	ret

[bits 32]

BEGIN_PM:
	mov ebx, MSG_PROTECTED_MODE
	call print_ascii_pm			; print string in 32 bit protected mode

	call KERN_OFFSET		; we call our kernel code located at KERNEL_OFFSET

	jmp $

BOOT_DRIVE:
	db 0 
MSG_REAL_MODE:
	db "Hello World", 0
MSG_PROTECTED_MODE:
	db "Hello Protected World", 0
MSG_LOAD_KERN:
	db "Loading kernel into memory", 0

times 510-($-$$) db 0
dw 0xaa55

