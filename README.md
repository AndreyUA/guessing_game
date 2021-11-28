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
