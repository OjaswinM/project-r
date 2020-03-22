# Project R 

A simple PoC OS (that will soon be)written in Rust. Currently, the kernel is implemented in C and little assembly.

## Usage

Although this OS is capable of booting on any system with GRUB, for convinience, we use an x86_64 system emulator like QEMU for development and testing purposes.  

Follow the following steps to boot the OS with QEMU on your device:-
  - Download the appropriate QEMU version for your OS from the [download page](https://www.qemu.org/download/).
  - Download `kernel.img` from the release page [here](https://github.com/OjaswinM/project-r/releases).
  - Run the following command from the terminal or Powershell:-
  ```
  qemu-system-x86_64 -kernel /path/to/kernel.img
  ```
This should open a new QEMU window which will boot the OS and start execution.

## Build

*Note: This section assumes you have correctly installed QEMU put it in your $PATH*

You will need to follow these steps to correctly set up the development environment:- 
  - In order to build the kernel files, a GCC cross compiler and cross Binutils are needed. The instruction to install the same on the OS of your choice can be found [here](https://wiki.osdev.org/GCC_Cross-Compiler). 
  - After installing the neccessary files, make sure that the cross compiler is in your $PATH or windows equivalent.
  - Next you will need make to compile the neccessary files. It can be installed as follows:-
    *Ubuntu:* `sudo apt-get intall make`
    *Windows:* Donwload it [here](http://gnuwin32.sourceforge.net/packages/make.htm).
    *MacOS:* `brew install make`
  - Finally you can compile and run the kernel by typing following command in the project's directory:-
    `make && make run`


*TODO*: Complete this readme.
