#![allow(non_snake_case)]

use dll_proxy::proxy_dll;
use std::thread;
use std::time::Duration;

mod logic;
use logic::apply;

proxy_dll!("dinput8.dll");

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "system" fn DllMain(hinstDLL: usize, dwReason: u32, _lpReserved: *mut usize) -> i32 {
    match dwReason {
        DLL_PROCESS_ATTACH => unsafe {
            let path: String = match init_proxy(hinstDLL) {
                Ok(p) => p,
                Err(e) => panic!("Could not proxy dll: {e}"),
            };

            let main_thread: thread::JoinHandle<()> = thread::spawn(|| {
                thread::sleep(Duration::from_secs(3));
                let _ = apply();
            });

            1
        },
        
        DLL_PROCESS_DETACH => {
            1
        },
        
        _ => 0,
    }
}
