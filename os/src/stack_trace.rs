use core::{arch::asm, ptr};

use log::error;

pub fn show_stack() {
    error!("=== Stack trace from fp chain ===\n");
    unsafe {
        let mut fp: *const usize;
        asm!("mv {}, fp", out(reg) fp);
        while fp != ptr::null() {
            let ra = *fp.sub(1);
            let prev_fp = *fp.sub(2);
            error!("Return address: 0x{:016x}", ra);
            error!("Old stack pointer: 0x{:016x}\n", prev_fp);
            fp = prev_fp as *const usize;
        }
    }
    error!("=== End ===\n");
}
