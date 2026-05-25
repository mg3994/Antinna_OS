#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        Self {
            limit_low: (limit & 0xffff) as u16,
            base_low: (base & 0xffff) as u16,
            base_middle: ((base >> 16) & 0xff) as u8,
            access,
            granularity: ((limit >> 16) & 0x0f) as u8 | (gran & 0xf0),
            base_high: ((base >> 24) & 0xff) as u8,
        }
    }
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u32,
}

static mut GDT: [GdtEntry; 3] = [
    // Null segment
    GdtEntry::new(0, 0, 0, 0),
    // Code segment: base 0, limit 4GB, access 0x9A (present, ring 0, code, exec/read), gran 0xCF (4KB blocks, 32-bit)
    GdtEntry::new(0, 0xffffffff, 0x9A, 0xCF),
    // Data segment: base 0, limit 4GB, access 0x92 (present, ring 0, data, read/write), gran 0xCF (4KB blocks, 32-bit)
    GdtEntry::new(0, 0xffffffff, 0x92, 0xCF),
];

pub fn init() {
    unsafe {
        let ptr = GdtPtr {
            limit: (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16,
            base: (&raw const GDT) as u32,
        };
        core::arch::asm!(
            ".intel_syntax noprefix",
            "lgdt [{0}]",
            "push 0x08",
            "lea eax, [3f]",
            "push eax",
            "retf",
            "3:",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            ".att_syntax",
            in(reg) &ptr,
            out("eax") _,
            options(readonly, nostack, preserves_flags)
        );
    }
}
