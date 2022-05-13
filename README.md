# rust-stm32g431
Blinking example for STM32G431RB Nucleo Board written in Rust

## Dependencies
### Hardware
NUCLEO-STM32G431RB using STM32G431RB (Cortex M4)

### Software preparation for NixOS
Install Packages:
```Shell
environment.systemPackages = with pkgs; [ rustup stlink ];
```

Add udev rules for stlink
```Shell
services.udev.packages = [ pkgs.stlink];
```

Install thumb instructions for Cortex M4 core and prepare binutils
```Shell
rustup install stable
rustup default stable
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Building the target
```Shell
make
```

## Download firmware to the flash
1. Ensure USB cable is connected
2. Check if stlink was detected
```Shell
dmesg | tail
```
3. Actually perform the download
```Shell
make download
```
