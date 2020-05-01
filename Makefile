# ~~~~~~~~~~ Compilers and flags ~~~~~~~~~~~~~
# Rust compiler
CARGO = cargo
RUSTC  = rustc
RUSTC_FLAGS = --target $(RUST_TARGET_PATH)

LD_SCRIPT = linker.ld
LD = x86_64-elf-ld 
LD_FLAGS = -T $(LD_SCRIPT) -melf_i386

# assembler
AS = nasm
AS_FLAGS = -felf

# ~~~~~~~~~~~~ PATHS  ~~~~~~~~~~~~~
KERNEL = kernel.img
RUST_TARGET_PATH = ./x86-64-target.json
# KERN_SRC_FILES = $(wildcard ./src/*)
KERN_SRC_FILES = kernel_entry.o #kernel_entry_64.o
KERN_BIN_PATH = ./target

# ~~~~~~~~~~~~ QEMU ~~~~~~~~~~~~~
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio 

all: kernel

kernel: $(KERN_SRC_FILES)
	#$(CARGO) xbuild $(RUSTC_FLAGS)
	$(LD) $(LD_FLAGS) -o $(KERNEL) $^ 

%.o: %.asm
	$(AS) $(AS_FLAGS) -o $@ $<

run: all
	$(QEMU) $(QEMU_FLAGS) -kernel $(KERNEL) 

run_and_pause: all
	$(QEMU) $(QEMU_FLAGS) -s -S -kernel $(KERNEL) 

clean:
	rm -f *.o *.img

