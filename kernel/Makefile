CXX = gcc		# C compiler
CXXFLAGS = -ffreestanding -m32		# C compiler flags
KERN_LOAD_ADDR = 0x1000
LD = ld
LD_FLAGS = -Ttext $(KERN_LOAD_ADDR) --oformat binary -m elf_i386
AS = nasm		# assembler
ASFLAGS = -f elf		# assembler flags

all: kernel

kernel: kernel_entry.o kernel.o  
	$(LD) $(LD_FLAGS) -o kernel.bin $^

%.o: %.c
	$(CXX) $(CXXFLAGS) -o $@ -c $< 

%.o: %.asm
	$(AS) $(ASFLAGS) -o $@ $< 
# kernel_entry.o: kernel_entry.asm
# 	$(AS) $(ASFLAGS) -o $@ $^
# 
# kernel.o: kernel.c
# 	$(CXX) $(CXXFLAGS) -c $< -o $@
# 
clean:
	rm *.o *.bin
