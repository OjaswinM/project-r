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
.section multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

/*
C assumes the stack to be already set up so we reserve 16KiB for the stack here,
since it needs to be 16byte aligned 
*/
.section bss
.align 16
stack_bottom:
	.skip 16384		# 16 KiB
stack_top:

/*
Out .text section. It must contain a _start label which is assigned as
the entrypoint to the kernel by the linker. 
*/
.section .text
.global _start
.type _start, @function
_start:

	/* 
	Now we set up the stack registers to point to the the stack which we had earlier 
	reserved
	*/
	mov $stack_bottom, %ebp
	mov $stack_top, %esp

	call main		# call main() which is linked here from C code

	# infinite looping in case main() ever returns
	cli
1:
	hlt
	jmp 1b
	
.size _start, . - _start
