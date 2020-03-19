MAKE = make
BOOTDIR = boot/
KERNDIR = kernel/
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio

build: 
	$(MAKE) -C $(BOOTDIR) 
	$(MAKE) -C $(KERNDIR)
	cat boot/bootloader kernel/kernel.bin > ./os.img
run: 
	$(QEMU) $(QEMU_FLAGS) os.img
