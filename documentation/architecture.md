# Antinna_OS Architecture

## Vision
Antinna_OS is designed to be an "AI-Native" operating system. Unlike traditional OSs where AI is an application-level add-on, Antinna_OS integrates AI concepts directly into the core management of the system.

## Core Principles
1. **Native over Wrappers**: No Google-based frameworks or languages. Focus on direct hardware interaction and native performance.
2. **Rust-Based**: Leveraging Rust's memory safety and performance for kernel and user-space development.
3. **Advanced Hardware Handling**: Manual GDT and IDT initialization for fine-grained control.
4. **AI-Driven Scheduling**: Future versions will use reinforcement learning to optimize process scheduling.
5. **Intelligent Memory Management**: Predictive paging based on user behavior patterns.
6. **Natural Interaction**: Built-in NLP and voice recognition as primary interface methods.

## Current Scaffolding
- **Bootloader**: Simple Multiboot-compliant entry point.
- **Kernel**: Rust kernel featuring:
    - Modular VGA text driver.
    - Global Descriptor Table (GDT) and TSS-ready structure.
    - Interrupt Descriptor Table (IDT) with exception handling.
    - PIC remapping and PS/2 Keyboard driver with hardware interrupts.
- **User Space**: Rust scaffolding for AI-driven applications (e.g., `chatbot`).

## Roadmap
- Implement memory management (Paging/Segmentation).
- Develop native drivers for disk (ATA/IDE).
- Integrate a lightweight AI inference engine into the kernel/user space boundary.
