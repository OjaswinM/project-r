# ~~~~~~~~~~ Compilers and flags ~~~~~~~~~~~~~
# Rust compiler
CARGO = cargo 
RUSTC  = rustc
CARGO_FLAGS = --target $(RUST_TARGET)

# ~~~~~~~~~~~~ PATHS & FILES ~~~~~~~~~~~~~
RUST_TARGET = ./x86_64-target.json
RUST_SRCS = $(wildcard ./src/*.rs)
KERN_BIN = target/x86_64-target/debug/bootimage-project-r.bin
# ~~~~~~~~~~~~ QEMU ~~~~~~~~~~~~~
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio 

all: run

run: kernel
	$(QEMU) $(QEMU_FLAGS) -drive format=raw,file=$(KERN_BIN) 

run_and_pause: iso
	$(QEMU) $(QEMU_FLAGS) -s -S -drive format=raw,file=$(KERN_BIN) 

kernel: $(RUST_SRCS)
	$(CARGO) bootimage $(CARGO_FLAGS)

clean:
	rm -rf target

