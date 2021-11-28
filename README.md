# Compiled file for Linux x86 64bit:
## https://drive.google.com/file/d/1VRYgd13VjAaQfipljr3NU8J98goa3bj-/view?usp=sharing
# Compiled file for Windows x86 64bit:
## https://drive.google.com/file/d/1-UF1WCtmNZ2zKFHqx9nlgHoohnqsyIok/view?usp=sharing

To see all platforms run:
### rustup target list

To add platform run:
### rustup target add PLATFORM_NAME

To add ability to compile fow Windows on Linux run:
### sudo apt-get install gcc-mingw-w64

and then run:
### cargo build --target x86_64-pc-windows-gnu --release

To compile for Linux run:
### cargo build --release
