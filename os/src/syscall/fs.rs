use crate::batch::{APP_BASE_ADDRESS, APP_SIZE_LIMIT,USER_STACK};

const FD_STDOUT: usize = 1;
const STACK_SIZE: usize = 0x1000;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let start_addr = buf as usize;
            let end_addr = start_addr.checked_add(len).unwrap_or(usize::MAX);
            if (start_addr >= APP_BASE_ADDRESS && end_addr <= APP_BASE_ADDRESS + APP_SIZE_LIMIT)
                || (start_addr >= USER_STACK.get_sp() - STACK_SIZE as usize&& end_addr <= USER_STACK.get_sp() )
                && start_addr <= end_addr {  // 防止整数溢出
                    let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                    let str = core::str::from_utf8(slice).unwrap();
                    print!("{}", str);
                    len as isize
            } else {
                -1
            }

            // let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            // let str = core::str::from_utf8(slice).unwrap();
            // print!("{}", str);
            // len as isize
        },
        _ => {
            -1
        }
    }
}