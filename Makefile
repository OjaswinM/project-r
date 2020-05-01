# ~~~~~~~~~~ Compilers and flags ~~~~~~~~~~~~~
# Rust compiler
CARGO = cargo
RUSTC  = rustc
RUSTC_FLAGS = --target $(RUST_TARGET_PATH)

LD_SCRIPT = linker.ld
LD = x86_64-elf-ld 
LD_FLAGS = -T $(LD_SCRIPT) -n --gc-sections

# assembler
AS = nasm
AS_FLAGS = -felf64

# ~~~~~~~~~~~~ PATHS & FILES ~~~~~~~~~~~~~
RUST_TARGET = ./x86-64-target.json
KERNEL = build/kernel.img
KERN_ISO = build/project-r.iso
GRUB_CFG = src/arch/x86_64/grub.cfg


ASM_SRC_FILES = $(wildcard ./src/arch/x86_64/*.asm)
ASM_OBJ_FILES = $(patsubst ./src/arch/x86_64/%.asm, ./build/arch/x86_64/%.o, $(ASM_SRC_FILES))

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

kernel: $(ASM_OBJ_FILES)
	#$(CARGO) xbuild $(RUSTC_FLAGS)
	$(LD) $(LD_FLAGS) -o $(KERNEL) $^ 

build/arch/x86_64/%.o: src/arch/x86_64/%.asm
	mkdir -p $(shell dirname $@)
	$(AS) $(AS_FLAGS) -o $@ $<

%.o: %.asm
	$(AS) $(AS_FLAGS) -o $@ $<

clean:
	rm -rf build

