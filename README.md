# Project R 

A PoC OS written in Rust. 

## Booting the OS

Although this OS is capable of booting on any x86_64 hardware, for convinience, we use an x86_64 system emulator like QEMU for development and testing.  

Follow the following steps to boot the OS with QEMU on your device:-
  - Download the appropriate QEMU version for your OS from the [download page](https://www.qemu.org/download/).
  - Download the most recent release from [here](https://github.com/OjaswinM/project-r/releases).
  - Run the following command from the terminal or Powershell:-
    ```
    qemu-system-x86_64 -drive format=raw,file=path/to/kernel
    ```
This should open a new QEMU window which will boot the OS and start execution.

## Build

*Note: This section assumes you have correctly installed QEMU and put it in your $PATH. If not, refer to the first step under "Booting the OS" section*

You will need to follow these steps to correctly set up the development environment:- 
  - Install Rust (preferably via rustup). More details can be found [here](https://www.rust-lang.org/tools/install)
  - We use some experimental features of Rust in our kernel which will only be available in the nightly release. Run `rustup override add nightly` to install Rust nightly.
  - Run `rustup component add rust-src` followed by `cargo install cargo-xbuild bootimage` to install the dependencies of the kernel.
  - Install `make` to automate building and launching the kernel. It can be installed as follows:-
    - Ubuntu: 
      ```
      sudo apt-get intall make
      ```
    - Windows: Donwload it [here](http://gnuwin32.sourceforge.net/packages/make.htm).
    - MacOS:
      ```
       brew install make
      ```
  - Finally you can compile and boot the kernel with `qemu` by running the following command in the project's root directory:-
    ```
    make run
    ```

*TODO*: Complete this readme.
