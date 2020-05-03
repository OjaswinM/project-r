ENTRY(start)

SECTIONS
{
	/*
	 * Leave initial 1MB for the bootloader and start loading kernel from 1MB
	 */ 
	. = 1M; 

	.boot :
	{
		KEEP(*(.multiboot))
	}

	.text BLOCK(4K): ALIGN(4K)
	{
		KEEP(*(.multiboot))
		*(.text)
	}

	/*
	 * Read only data
	 */
	.rodata BLOCK(4K): ALIGN(4K)
	{
		*(.rodata)
	}

	.data BLOCK(4K): ALIGN(4K)
	{
		*(.data)
	}

	.bss BLOCK(4K): ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}

}
