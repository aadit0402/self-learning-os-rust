#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/* Now we have added our own println, so we can use this here and will not need to add here
 * anything like write_str or writer_lock. For using println, We made it public but hidden from 
 * generated documentation.
 */

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hellow World {}", "@");
    
    loop {}
}

