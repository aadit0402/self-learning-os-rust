#![no_std]
#![no_main]

use core::panic::PanicInfo;
use self_learning_os::{QemuExitCode, serial_print, exit_qemu, serial_println};



/*#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/*pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running from should_panic.rs: {}", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}*/


fn should_fail() {
    serial_print!("should_fail...");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("OK");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

