# ~~~~~~~~~~ Compilers and flags ~~~~~~~~~~~~~
# C compiler
CXX = i686-elf-gcc		
CXXFLAGS = -ffreestanding -std=gnu99 -g -O2 -Wall -Wextra -I $(DRIVERS_DIR)	# C compiler flags
LD_SCRIPT = linker.ld
LD = i686-elf-gcc
LD_FLAGS = -T $(LD_SCRIPT) -ffreestanding -O2 -nostdlib 

# assembler
AS = i686-elf-as		
AS_FLAGS = -g		

# ~~~~~~~~~~~~ Dependencies ~~~~~~~~~~~~~
DRIVERS = $(wildcard ../drivers/*.c)
DRIVERS_OBJ = $(wildcard ../drivers/*.o)
DRIVERS_DIR = ../drivers/

KERNEL = ./kernel.img

# ~~~~~~~~~~~~ QEMU ~~~~~~~~~~~~~
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio 

all: kernel

kernel: kernel_entry.o kernel.o 
	$(LD) $(LD_FLAGS) -o $(KERNEL) $^ -lgcc

%.o: %.c
	$(CXX) $(CXXFLAGS) -o $@ -c $< 

# %.o: %.asm
# 	$(AS) $(ASFLAGS) -o $@ $< 

%.o: %.s
	$(AS) $(AS_FLAGS)$< -o $@

run: all
	$(QEMU) $(QEMU_FLAGS) -kernel $(KERNEL) 

run_and_pause: all
	$(QEMU) $(QEMU_FLAGS) -s -S -kernel $(KERNEL) 

clean:
	rm -f *.o *.img
