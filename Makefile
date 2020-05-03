# ~~~~~~~~~~ Compilers and flags ~~~~~~~~~~~~~
# Rust compiler
CARGO = cargo 
RUSTC  = rustc
RUSTC_FLAGS = --target $(RUST_TARGET)

LD_SCRIPT = ./src/arch/x86_64/linker.ld
LD = x86_64-elf-ld 
LD_FLAGS = -T $(LD_SCRIPT) -n --gc-sections -g

# assembler
AS = nasm
AS_FLAGS = -felf64 -g -F dwarf

# ~~~~~~~~~~~~ PATHS & FILES ~~~~~~~~~~~~~
RUST_TARGET = ./src/arch/x86_64/x86_64-target.json
KERNEL = build/kernel.img
KERN_ISO = build/project-r.iso
GRUB_CFG = src/arch/x86_64/grub.cfg
RUST_OBJ = target/x86_64-target/debug/project-r

RUST_SRCS = $(wildcard ./src/*.rs)
ASM_SRCS = $(wildcard ./src/arch/x86_64/*.asm)
ASM_OBJS = $(patsubst ./src/arch/x86_64/%.asm, ./build/arch/x86_64/%.o, $(ASM_SRCS))

# ~~~~~~~~~~~~ QEMU ~~~~~~~~~~~~~
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio 

all: run

run: iso
	$(QEMU) $(QEMU_FLAGS) -cdrom $(KERN_ISO) 

run_and_pause: iso
	$(QEMU) $(QEMU_FLAGS) -s -S -cdrom $(KERN_ISO) 

iso: kernel
	mkdir -p build/isofiles/boot/grub
	cp $(KERNEL) build/isofiles/boot/kernel.bin
	cp $(GRUB_CFG) build/isofiles/boot/grub/grub.cfg
	grub-mkrescue -o $(KERN_ISO) build/isofiles
	rm -rf build/isofiles

kernel: $(ASM_OBJS) $(RUST_OBJ)
	$(LD) $(LD_FLAGS) -o $(KERNEL) $^ 

$(RUST_OBJ): $(RUST_SRCS)
	$(CARGO) xbuild $(RUSTC_FLAGS)

build/arch/x86_64/%.o: src/arch/x86_64/%.asm
	mkdir -p $(shell dirname $@)
	$(AS) $(AS_FLAGS) -o $@ $<

%.o: %.asm
	$(AS) $(AS_FLAGS) -o $@ $<

clean:
	rm -rf build

