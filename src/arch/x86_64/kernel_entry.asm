; A small assembly file that is performs the following:-
; 
; - sets up stack
; - sets up PAE paging and identity maps first 2MB of memory
; - sets up a GDT
; - enables long mode

section .bss
bits 32
align 4096
pml4_table:
	resb 4096
pdp_table:
	resb 4096
pd_table:
	resb 4096
page_table:
	resb 4096
stack_bottom:
	; 16 KiB
	resb 16384		
stack_top:

section .rodata
gdt_long_mode:
		dq 0
gdt_null:  			     ; The null descriptor.
	dw 0xFFFF                    ; Limit (low).
	dw 0                         ; Base (low).
	db 0                         ; Base (middle)
	db 0                         ; Access.
	db 1                         ; Granularity.
	db 0                         ; Base (high).
gdt_code_entry: 
	dw 0                         ; Limit (low).
	dw 0                         ; Base (low).
	db 0                         ; Base (middle)
	db 10011010b                 ; Access (exec/read).
	db 10101111b                 ; Granularity, 64 bits flag, limit19:16.
	db 0                         ; Base (high).
gdt_data_entry:
	dw 0                         ; Limit (low).
	dw 0                         ; Base (low).
	db 0                         ; Base (middle)
	db 10010010b                 ; Access (read/write).
	db 00000000b                 ; Granularity.
	db 0                         ; Base (high).
gdt_pointer:
	dw $ - gdt_long_mode - 1
	dq gdt_long_mode

CODE_SEG equ gdt_code_entry - gdt_long_mode
DATA_SEG equ gdt_data_entry - gdt_long_mode

; The .text section. It must contain a _start label which is assigned as
; the entrypoint to the kernel by the linker. 
section .text
set_up_page_tables:
	; We will start with a basic paging structure annd identity mapping
	; the first 2MBs
	; The 4 paging tables used are:-
	; 
	; - pml4 table 
	; - pdp table 
	; - pd table 
	; - page table 
	mov ebx, pdp_table
	
	; Set R/W and P bit for the pml4 entry
	add ebx, 0x3

	; Point the first entry in pml4 to address of first entry of pdp
	mov [pml4_table], ebx
	
	; Repeat to connect pdp to pd
	mov ebx, pd_table
	add ebx, 0x3
	mov [pdp_table], ebx

	mov ebx, page_table
	add ebx, 0x3
	mov [pd_table], ebx

	; Finally set the 512 64bit entries of the page table to 
	; point to sequntial blocks of 4KB each thus mapping 2MBs
	mov eax, page_table
	mov ebx, 0x00000003
	mov ecx, 0

.init_pte:
	mov [eax], ebx
	add ebx, 0x1000
	add eax, 8

	inc ecx
	cmp ecx, 512
	jne .init_pte

	ret

enable_long_mode_paging:
	; To enable paging we need to perform the following operations:-
	; 
	; - store start address of pml4 table in cr3 register
 	; - enable Physical Address Extension(PAE) flag in cr4 register
	; - set long mode bit in EFER register
	; - enable paging
	mov eax, pml4_table
	mov cr3, eax

	; set PAE flag 
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; set the long mode bit in EFER MSR(0xc0000080) 
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr
	
	; enable paging in cr0 
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret

extern long_mode_start
global start
start:

	; Now we set up the stack registers to point to the the stack which we 
	; had earlier reserved
	mov ebp, stack_bottom
	mov esp, stack_top

	call set_up_page_tables
	call enable_long_mode_paging

	lgdt [gdt_pointer]

	jmp CODE_SEG:long_mode_start

.inf:
	hlt
	jmp .inf
	
