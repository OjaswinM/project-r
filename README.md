# Project R 

A simple PoC OS (that will soon be)written in Rust. Currently, the kernel is implemented in C and little assembly.

## Usage

Although this OS is capable of booting on any system with GRUB, for convinience, we use an x86_64 system emulator like QEMU for development and testing purposes.  

Follow the following steps to boot the OS with QEMU on your device:-
  - Download the appropriate QEMU version for your OS from the [download page](https://www.qemu.org/download/).
  - Download `kernel/kernel.img` file present in this repository.
  - Run the following command from the terminal or Powershell:-
  ```
  qemu-system-x86_64 -kernel /path/to/kernel.img
  ```
This should open a new QEMU window which will boot the OS and start execution.

## Build

In order to build the kernel files, a GCC cross compiler and cross Binutils are needed. The instruction to install the same on the OS of your choice can be found [here](https://wiki.osdev.org/GCC_Cross-Compiler).

*TODO*: Complete this readme.
