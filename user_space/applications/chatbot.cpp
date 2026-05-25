/**
 * Antinna_OS AI Chatbot Stub
 * This is a user-space application demonstrating the AI-native vision.
 *
 * Refactored to be freestanding-compatible.
 * Real AI logic will be integrated via syscalls in the future.
 */

extern "C" void _start() {
    const char* welcome_msg = "--- Antinna_OS AI Chatbot (Native) ---";
    (void)welcome_msg; // Avoid unused warning for now

    // In a real OS, we would use a syscall like write(1, msg, len)
    // For now, this serves as a structural placeholder.

    while(1) {
        // Wait for native events
    }
}
