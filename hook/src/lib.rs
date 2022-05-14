use std::mem;
use std::os::raw::{c_int, c_uint, c_void};

//被hook的系统函数
#[no_mangle]
pub extern "C" fn sleep(i: c_uint) -> c_uint {
    println!("hooked {}", i);
    //获取原始系统函数
    let original = unsafe {
        mem::transmute::<_, extern "C" fn(c_uint) -> c_uint>(libc::dlsym(libc::RTLD_NEXT, "sleep".as_ptr() as _))
    };
    //调用原始系统函数
    original(i)
}