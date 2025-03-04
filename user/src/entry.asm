# os/src/entry.asm
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    # 这里不应该调用rust_main，而是跳转到入口
    # 由于_start由lib.rs实现，这个汇编入口点应该直接设置栈指针
    # 不调用任何函数

    .section .bss.stack
    .global boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
