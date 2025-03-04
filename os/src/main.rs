#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod logging;
mod config;


global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("link_app.S"));

// 标记为不被修改的入口函数
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        safe fn stext(); // begin addr of text segment
        safe fn etext(); // end addr of text segment
        safe fn srodata(); // start addr of Read-Only data segment
        safe fn erodata(); // end addr of Read-Only data ssegment
        safe fn sdata(); // start addr of data segment
        safe fn edata(); // end addr of data segment
        safe fn sbss(); // start addr of BSS segment
        safe fn ebss(); // end addr of BSS segment
        safe fn boot_stack_lower_bound(); // stack lower bound
        safe fn boot_stack_top(); // stack top
    }

    // 清零.bss段内存
    clear_bss();
    // 在这里可以添加更多的初始化代码
    let config = config::Config::parse_from_cmdline();
    logging::init(config.log_level, config.log_en);
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    error!("Hello, Navi!");
    warn!("Hello, Lain!");
    info!("Hello, Mf1bzz!");
    debug!("Hello, Free!");
    trace!("Hello, World!");

    panic!("Shutdown machine!");
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    // 将.bss段内存清零
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

