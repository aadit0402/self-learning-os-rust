#![no_main]
#![no_std]

#![feature(custom_test_frameworks)]
#![test_runner(self_learning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]




use core::panic::PanicInfo;
use self_learning_os::println;

/*#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hellow World {}", "@");
   
    self_learning_os::init(); //to enable interrupt descriptor table. Here I am going to test breakpoint exception.
    x86_64::instructions::interrupts::int3();
    
   unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

   fn stack_overflow() {
       stack_overflow();
   }
    
   stack_overflow();
    #[cfg(test)]
    test_main();
	/* this is the test framework entry function */
    
    println!("It did not crash!!!");
    loop {}
}


/*#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion...");
    assert_eq!(1,1);
    serial_println!("[ok]");
}

*/
/* here &[&dyn fn()]: is a argument, slice of trait object references of the fn() trait.
 */

/*#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) { 
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success); /* now this call will shutdown the kernel properly*/
}
*/

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/*#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}*/
/* Now we have added our own println, so we can use this here and will not need to add here
 * anything like write_str or writer_lock. For using println, We made it public but hidden from 
 * generated documentation.
 */

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    self_learning_os::test_panic_handler(info);
}

