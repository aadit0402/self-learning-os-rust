use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;

//initially I used static IDT, but then we were trying to modify this IDT value, So I tried this
//mut keyword, so that we can modify the value. Point to remember is that static mutable is prone
//to data race error, That is the reason we need to use unsafe block. 

//static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}
pub fn init_idt() {
    //let mut idt = InterruptDescriptorTable::new();
    //IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("Exception: Breakpoint!!!\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler( 
stack_frame: &mut InterruptStackFrame, _error_code: u64) 
{ 
     panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame); 
}

//here we are creating a test for testing breakpoint exceptions.

#[cfg(test)]
use crate:: {serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception in lib.rs...");
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}

