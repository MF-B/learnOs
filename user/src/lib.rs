#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod syscall;  // 确保有一个 syscall.rs 文件
mod lang_items;

pub use console::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }

