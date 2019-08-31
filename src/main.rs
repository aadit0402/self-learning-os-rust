#![no_main]
#![no_std]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hellow World {}", "@");
    
    #[cfg(test)]
    test_main();
	/* this is the test framework entry function */

    loop {}
}


#[test_case]
fn trivial_assertion() {
   print!("trivial assertion...");
    assert_eq!(1,1);
    println!("[ok]");
}

/* here &[&dyn fn()]: is a argument, slice of trait object references of the fn() trait.
 */

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) { 
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/* Now we have added our own println, so we can use this here and will not need to add here
 * anything like write_str or writer_lock. For using println, We made it public but hidden from 
 * generated documentation.
 */

