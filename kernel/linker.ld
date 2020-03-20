ENTRY(_start)

SECTIONS
{
	/*
	 * Leave initial 1MB for the bootloader and start loading kernel from 1MB
	 */ 
	. = 1M; 

	.text BLOCK(4K): ALIGN(4K)
	{
		*(.multiboot)
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
