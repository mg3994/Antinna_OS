# Compilation settings
AS = as
CXX = g++
LD = ld

ASFLAGS = --32
CXXFLAGS = -m32 -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti
LDFLAGS = -m elf_i386 -T kernel/linker.ld

# Files
BOOT_SRC = bootloader/source/boot.S
KERNEL_SRC = kernel/source/kernel.cpp
BOOT_OBJ = boot.o
KERNEL_OBJ = kernel.o
KERNEL_BIN = kernel.bin

.PHONY: all clean

all: $(KERNEL_BIN)

$(BOOT_OBJ): $(BOOT_SRC)
	$(AS) $(ASFLAGS) $< -o $@

$(KERNEL_OBJ): $(KERNEL_SRC)
	$(CXX) -c $(CXXFLAGS) $< -o $@

$(KERNEL_BIN): $(BOOT_OBJ) $(KERNEL_OBJ)
	$(LD) $(LDFLAGS) $^ -o $@

clean:
	rm -f $(BOOT_OBJ) $(KERNEL_OBJ) $(KERNEL_BIN)
