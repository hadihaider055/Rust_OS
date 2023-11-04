- #### Run Cargo Build with the command

`cargo build`

- #### To create the bootable disk image

`cargo bootimage`

- #### To run the OS

`qemu-system-x86_64 -drive format=raw,file=C:\Users\haide\Documents\Development\Rust_OS\target\x86_64-blog_os\debug\bootimage-Rust_OS.bin`
