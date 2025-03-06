use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use core::arch::asm;
//use core::num;
use lazy_static::*;
use log::*;

const USER_STACK_SIZE: usize = 4096 * 8;
const KERNEL_STACK_SIZE: usize = 4096 * 8;
const MAX_APP_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}
impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}


impl AppManager {
    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            error!("All applications completed!");
            shutdown(false);
        }
        // 输出log信息
        info!("[kernel] Loading app_{}", app_id);
        // 清空运行app的内存区域(填0)
        unsafe {
            (APP_BASE_ADDRESS..APP_BASE_ADDRESS + APP_SIZE_LIMIT).for_each(|a| {
                (a as *mut u8).write_volatile(0)
            });    
            // 复制app的代码到运行app的内存区域
            let app_src = self.app_start[app_id];
            let app_dst = APP_BASE_ADDRESS;
            let app_size = self.app_start[app_id + 1] - app_src;
            core::ptr::copy(app_src as *const usize, app_dst as *mut usize, app_size / core::mem::size_of::<usize>());
            // memory fence about fetching the instruction memory
            asm!("fence.i");
        }
    }
}


// 定义全局变量
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe { 
        UPSafeCell::new({
            // 获取app数组的指针
            unsafe extern "C" { 
                fn _num_app(); 
            }
            let mut num_app_ptr = _num_app as usize as *const usize;
            // 获取app的数量
            let num_app = num_app_ptr.read_volatile();
            // 获取各个app的起始地址数组并返回给APP_MAMAGER
            let mut app_start:[usize;MAX_APP_NUM+1] = [0; MAX_APP_NUM + 1];
            for i in 0..num_app+1 {
                app_start[i] = {
                    num_app_ptr = num_app_ptr.add(1);
                    *num_app_ptr
                }
            }
            // 设置返回值
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

// 初始化批处理系统
pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    // 引用app_manager,加载程序
    let mut app_manager = APP_MANAGER.exclusive_access();
    app_manager.load_app(app_manager.get_current_app());
    app_manager.move_to_next_app();
    // 释放app_manager
    drop(app_manager);
    // 准备并跳转到用户应用
    unsafe extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp()
        )) as *mut TrapContext as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}