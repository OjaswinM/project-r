# a small assembly file that is linked at the beginning of the kernel. It contains
# a multiboot header required by grub to boot out kernel. It simply calls the
# main() routine which is the entry point of the actual kernel.
# the kernel
.code32 # since we are in protected mode

.set ALIGN, 1 << 0		# to tell GRUB to load out modules at page aligned
											# boundaries
.set MEM_INFO, 1 << 1		# provide memory map
.set FLAGS, ALIGN | MEM_INFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

# multiboot header for GRUB
.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

/*
C assumes the stack to be already set up so we reserve 16KiB for the stack here,
since it needs to be 16byte aligned 
*/
.section .bss
.align 4096
pml4_table:
	.skip 4096
pdp_table:
	.skip 4096
pd_table:
	.skip 4096
page_table:
	.skip 4096
stack_bottom:
	.skip 16384		# 16 KiB
stack_top:

.section .rodata
gdt_long_mode:
	.long 0
	.long 0
gdt_code_entry:
	.word 0x0
	.word 0x0
	.byte 0
	.byte 0b10011010
	.byte 0b10101111
	.byte 0
gdt_pointer:
	.word gdt_pointer - gdt_long_mode - 1
	.long 0
	.long gdt_long_mode

CODE_SEG = gdt_long_mode-gdt_code_entry

/*
The .text section. It must contain a _start label which is assigned as
the entrypoint to the kernel by the linker. 
*/
.section .text

set_up_page_tables:

	/*
	We will start with a basic paging structure annd identity mapping
	the first 2MBs
	The 4 paging tables used are:-
	
	- pml4 table 
	- pdp table 
	- pd table 
	- page table 
	*/
	mov $pml4_table, %eax
	mov $pdp_table, %ebx
	
	/*
	Set R/W and P bit for the pml4 entry
	*/
	add $0x3, %ebx

	/*
	Point the first entry in pml4 to address of first entry of pdp
	*/
	mov %ebx, (%eax)
	
	/*
	Repeat to connect pdp to pd
	*/
	mov $pdp_table, %eax
	mov $pd_table, %ebx
	add $0x3, %ebx
	mov %ebx, (%eax)

	mov $pd_table, %eax
	mov $page_table, %ebx
	add $0x3, %ebx
	mov %ebx, (%eax)

	/*
	Finally set the 512 64bit entries of the page table to 
	point to sequntial blocks of 4KB each thus mapping 2MBs
	*/
	mov $page_table, %eax
	mov $0x00000003, %ebx
	mov $512, %ecx

init_pte:
	mov %ebx, (%eax)
	add $0x1000, %ebx
	add $8, %eax
	loop init_pte

	ret

enable_long_mode_paging:
	/*
	To enable paging we need to perform the following operations:-
	
	- store start address of pml4 table in cr3 register
 	- enable Physical Address Extension(PAE) flag in cr4 register
	- set long mode bit in EFER register
	- enable paging
	*/
	
	mov %cr3, %eax
	or pml4_table, %eax
	mov %eax, %ecx
	mov %eax, %cr3

	/* set PAE flag */
	mov %cr4, %eax
	xor %ebx, %ebx
	mov $1, %ebx
	shl $5, %ebx
	or %ebx, %eax
	mov %eax, %cr4

	/* set the long mode bit in EFER MSR(0xc0000080) */
	mov $0xc0000080, %ecx
	rdmsr
	xor %ebx, %ebx
	mov $0x01, %ebx
	shl $8, %ebx
	or %ebx, %eax
	wrmsr
	
	/* enable paging in cr0 */
	mov %cr0, %eax
	xor %ebx, %ebx
	mov $1, %ebx
	shl $31, %ebx
	or %ebx, %eax
	mov %eax, %cr0

	ret

# .extern long_mode_start
.global _start
.type _start, @function
_start:

	/* 
	Now we set up the stack registers to point to the the stack which we 
	had earlier reserved
	*/
	mov $stack_bottom, %ebp
	mov $stack_top, %esp

	call set_up_page_tables
	call enable_long_mode_paging

	lgdt gdt_pointer

	# jmp $CODE_SEG_OFFSET, $long_mode_start

okay:
	mov $0x2f4b2f4f, %eax
 	mov %eax, (0xb8000)

	/* call main() which is linked here from C code */
	/*call main		*/

	/* infinite looping in case main() ever returns */
	cli
1:
	hlt
	jmp 1b
	
.size _start, . - _start
