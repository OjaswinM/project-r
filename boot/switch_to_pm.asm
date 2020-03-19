[bits 16]
; switch to protected mode(pm) which supports 32 bit instructions
switch_to_pm:

	cli		; disable interrupts till we set up pm. This is done because when the OS
				; boots in real mode, the BIOS sets up an Interrupt Vector Table(IVT)
				; for common tasks like printing to the screen. Entries in these tables
				; point to the Interrupt Service Routines which are in 16 bit. When we 
				; switch to 32 bit, we render this IVT useless and this we temporarily 
				; stop interrupts till we can handle them properly again.

	lgdt [gdt_descriptor]

; indirectly set the first bit of cr0 control register to indicate shift to 32
; bit protected mode
	mov eax, cr0
	or eax, 0x1
	mov cr0, eax

;	In protected mode, the segment registers are interpreted as offsets into the 
; GDT which we loaded with lgdt command. So to simply perform a long jump, we 
; the offset to the code segment followed by a known label in the code segment
; After this jump, all the instructions will be processed by the 32 bit 
; circuitry ie. the CPU will run in 32 bit protected mode
	jmp CODE_SEG:init_pm

; From hereon, all code will be proccessed in 32 bit protected mode
[bits 32]

init_pm:
; We set all segments to point to the DATA_SEG
		mov ax, DATA_SEG
		mov ds, ax
		mov ss, ax
		mov es, ax
		mov fs, ax
		mov gs, ax

		mov ebp, 0x90000		; stack pointer points to the top of free space
		mov esp, ebp
		
		call BEGIN_PM		; call a well known label
