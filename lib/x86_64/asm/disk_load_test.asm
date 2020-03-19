[org 0x7c00]

mov [BOOT_DRIVE], dl		; BIOS stores our boot drive in dl on boot
xor dx, dx

mov bp, 0x8000
mov sp, bp

mov bx, 0x1000
mov dh, 1
mov dl, [BOOT_DRIVE]
call disk_load

mov dx, [0x1000]
call print_ascii

jmp $

%include "disk_load.asm"
%include "print_ascii.asm"

BOOT_DRIVE: 
	db 0

times 510-($-$$) db 0

dw 0xaa55

db "HELLLO",0

