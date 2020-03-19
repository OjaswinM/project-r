CXX = gcc		# C compiler
CXXFLAGS = -Wall -g 		# C compiler flags
AS = nasm		# assembler
ASFLAGS = -f bin		# assembler flagas
ASM_LIBS_X86 = dev/lib/x86_64/asm/		# assembler lib file path
VPATH = $(PROJ)/$(ASM_LIBS_X86)

QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio 

all: bootloader

bootloader: bootloader.asm print_ascii.asm gdt.asm print_ascii_pm.asm switch_to_pm.asm
	$(AS) $(ASFLAGS) -i $(PROJ)$(ASM_LIBS_X86) -o bootloader bootloader.asm

run: 
	$(QEMU) $(QEMU_FLAGS) bootloader

clean:
	rm bootloader
