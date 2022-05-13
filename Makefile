ELF = rust-stm32g431

all: target

target: src/main.rs
	cargo build --release --bin $(ELF)
	cargo size --release --bin $(ELF) -- -A

clean:
	cargo clean
download: all
	# Generate binary
	cargo size --release --bin $(ELF) -- -A
	cargo objcopy --release --bin $(ELF) -- -Obinary /tmp/$(ELF).bin
	# Upload to device
	st-flash --reset write /tmp/$(ELF).bin 0x08000000

