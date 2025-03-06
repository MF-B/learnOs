use crate::sbi::shutdown;
use core::panic::PanicInfo;
use log::error;
use crate::stack_trace::show_stack;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        error!("Panicked: {}", info.message());
    }
    show_stack();
    shutdown(true)
}