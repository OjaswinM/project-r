[bits 16]
; load DH sectors to ES:BX from drive DL
disk_load:
; Loads sectors from selected drive which is addressed using CHS addressing
; ~~~~~~~ REGISTERS ~~~~~~~~
; ah: 0x02 - the interrupt number for BIOS read sector function
; al: number of sectors to read/ sectors read after calling int 0x13
; bx: address to read the sectors to. The sector is read from es:bx
; dl: drive number to read
; dh: side of floppy. 0 indexed
; cl: sector number - 1 indexed
; ch: cylinder number
	push dx		; so we can later pop and check the number of sectors to read
	mov al, dh
	mov ah, 0x02
	mov ch, 0x0		; cylinder 0
	mov dh, 0x0		; head 0
	mov cl, 0x02	; start from sector 2 - 1 indexed

	int 0x13

	jc disk_error

	pop dx 
	cmp dh, al 
	jne disk_error
	ret

	disk_error:
		
		mov bx, DISK_ERROR_MSG
		call print_ascii
		jmp $

	; variables

	DISK_ERROR_MSG db "Disk read error!", 0 
