use crate::keyboard;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_high: u16,
}

impl IdtEntry {
    const fn new(handler: u32, selector: u16, flags: u8) -> Self {
        Self {
            base_low: (handler & 0xffff) as u16,
            selector,
            zero: 0,
            flags,
            base_high: ((handler >> 16) & 0xffff) as u16,
        }
    }
}

#[repr(C, packed)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::new(0, 0, 0); 256];

extern "x86-interrupt" fn breakpoint_handler(_sf: InterruptStackFrame) {
    // Breakpoint handler
}

extern "x86-interrupt" fn double_fault_handler(_sf: InterruptStackFrame, _error_code: u32) -> ! {
    loop {}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_sf: InterruptStackFrame) {
    unsafe {
        let scancode: u8 = inb(0x60);
        if let Some(_c) = keyboard::get_char(scancode) {
            // In a real OS with a global writer, we would print the char
        }

        // Send EOI to PIC
        outb(0x20, 0x20);
    }
}

// Port IO
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        ".intel_syntax noprefix",
        "out dx, al",
        ".att_syntax",
        in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        ".intel_syntax noprefix",
        "in al, dx",
        ".att_syntax",
        in("dx") port, out("al") val, options(nomem, nostack, preserves_flags));
    val
}

unsafe fn io_wait() {
    outb(0x80, 0);
}

pub fn init() {
    unsafe {
        // Remap PIC
        let master_command = 0x20;
        let master_data = 0x21;
        let slave_command = 0xA0;
        let slave_data = 0xA1;

        // ICW1: start initialization
        outb(master_command, 0x11);
        io_wait();
        outb(slave_command, 0x11);
        io_wait();

        // ICW2: set offset
        outb(master_data, 0x20); // Master offset 32
        io_wait();
        outb(slave_data, 0x28);  // Slave offset 40
        io_wait();

        // ICW3: tell master about slave
        outb(master_data, 0x04);
        io_wait();
        outb(slave_data, 0x02);
        io_wait();

        // ICW4: environment info
        outb(master_data, 0x01);
        io_wait();
        outb(slave_data, 0x01);
        io_wait();

        // Unmask keyboard interrupt (IRQ1)
        outb(master_data, 0xFD); // 0xFD = 1111 1101 (IRQ1 enabled)
        outb(slave_data, 0xFF);

        // Set up IDT entries
        IDT[3] = IdtEntry::new(breakpoint_handler as *const () as u32, 0x08, 0x8E);
        IDT[8] = IdtEntry::new(double_fault_handler as *const () as u32, 0x08, 0x8E);
        IDT[33] = IdtEntry::new(keyboard_interrupt_handler as *const () as u32, 0x08, 0x8E); // IRQ1 + 32

        let ptr = IdtPtr {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: (&raw const IDT) as u32,
        };
        core::arch::asm!(
            ".intel_syntax noprefix",
            "lidt [{}]",
            ".att_syntax",
            in(reg) &ptr);

        // Enable interrupts
        core::arch::asm!(
            ".intel_syntax noprefix",
            "sti",
            ".att_syntax",
            options(nomem, nostack, preserves_flags));
    }
}
