# Compilation settings
AS = as
LD = ld
CARGO = cargo

ASFLAGS = --32
LDFLAGS = -m elf_i386 -T kernel/linker.ld

# Files
BOOT_SRC = bootloader/source/boot.S
BOOT_OBJ = boot.o
KERNEL_LIB = kernel/target/i686-unknown-linux-gnu/release/libkernel.a
KERNEL_BIN = kernel.bin

.PHONY: all clean kernel_rust

all: $(KERNEL_BIN)

$(BOOT_OBJ): $(BOOT_SRC)
	$(AS) $(ASFLAGS) $< -o $@

kernel_rust:
	cd kernel && $(CARGO) build --release --target i686-unknown-linux-gnu

$(KERNEL_BIN): $(BOOT_OBJ) kernel_rust
	$(LD) $(LDFLAGS) $(BOOT_OBJ) $(KERNEL_LIB) -o $@

clean:
	rm -f $(BOOT_OBJ) $(KERNEL_BIN)
	cd kernel && $(CARGO) clean
