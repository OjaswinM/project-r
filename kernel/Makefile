CXX = i686-elf-gcc		# C compiler
CXXFLAGS = -ffreestanding -std=gnu99 -g -O2 -Wall -Wextra -I $(DRIVERS_DIR)	# C compiler flags
LD_SCRIPT = linker.ld
LD = i686-elf-gcc
LD_FLAGS = -T $(LD_SCRIPT) -ffreestanding -O2 -nostdlib 
AS = i686-elf-as		# assembler
AS_FLAGS = -g		# assembler flags

DRIVERS = $(wildcard ../drivers/*.c)
DRIVERS_OBJ = $(wildcard ../drivers/*.o)
DRIVERS_DIR = ../drivers/

QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio -s -S

all: kernel

kernel: kernel_entry.o $(DRIVERS_OBJ) kernel.o 
	$(LD) $(LD_FLAGS) -o kernel.img $^ -lgcc

%.o: %.c
	$(CXX) $(CXXFLAGS) -o $@ -c $< 

# %.o: %.asm
# 	$(AS) $(ASFLAGS) -o $@ $< 

%.o: %.s
	$(AS) $(AS_FLAGS)$< -o $@

run: all
	$(QEMU) $(QEMU_FLAGS) -kernel kernel.img

clean:
	rm -f *.o *.img
