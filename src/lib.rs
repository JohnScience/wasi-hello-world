#![no_std]
#![no_main]

use wasi::{fd_write, Ciovec, FD_STDOUT};

// extern "C" {
//     pub fn __main_void();
//     pub fn __wasm_call_dtors();
//     pub fn __wasi_proc_exit();
// }

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    fn proc_exit(code: u32);
}

#[no_mangle]
pub extern "C" fn __main_void() -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    let message = "Hello, world!\n";
    let data = {
        let buf: *const u8 = message.as_ptr();
        let buf_len: usize = message.len();
        [Ciovec { buf, buf_len }]
    };
    let _bytes_written = unsafe { fd_write(FD_STDOUT, &data) }.unwrap();
    proc_exit(0)
}

#[panic_handler]
// https://github.com/rust-lang/rust-analyzer/issues/4490#issuecomment-1074922003
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
