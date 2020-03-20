MAKE = make

# ~~~~~~~~~~~~~~ Dependencies ~~~~~~~~~~~~~
BOOT_DIR = boot/
KERN_DIR = kernel/
DRIVERS_DIR = drivers/

SRCS = $(wildcard $(KERN_DIR/*.c)) $(wildcard $(DRIVERS_DIR/*.c)) 
OBJS = $(patsubst %.c, %.o, $(SRC))

# ~~~~~~~~~~~~~~ QEMU ~~~~~~~~~~~~~
QEMU = qemu-system-x86_64
QEMU_FLAGS = -monitor stdio

build: all
	$(MAKE) -C $(DRIVERS_DIR) 
	$(MAKE) -C $(KERN_DIR)

run: all
	$(MAKE) -C $(KERN_DIR) $@

run_and_pause: all
	$(MAKE) -C $(KERN_DIR) $@

all: $(OBJS)

clean:
	$(MAKE) -C $(KERN_DIR) $@
	$(MAKE) -C $(DRIVERS_DIR) $@
