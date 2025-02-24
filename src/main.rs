#![no_std]
#![no_main]

// GPU
// 0xB880 - Mailbox offset
// 0x3F00_0000 - peripheral base

// 0x00 - READ
// 0x18 - STATUS
// 0x20 - WRITE 

use pi_zero2lib::{ display::Display, gpio::{ GPIOfunc, GPIO }, time::{ Duration, Timer } };
use core::arch::asm;

unsafe extern "C" {
    static _stack_start: usize;
    static mut __bss_start: usize;
    static mut __bss_end: usize;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "ldr x0, =_stack_start",  // Load the address of _stack_start into x0
            "mov sp, x0",             // Set the stack pointer (sp) to the value in x0
            options(nostack, nomem)
        );

        main();
    }
}


/* 
fn blink(n: u32) {
    for _ in 0..n {
        GPIO::set(29);
        Timer::delay(Duration::from_millis(100));

        // Off
        GPIO::clear(29);
        Timer::delay(Duration::from_millis(100));
    }
}
*/
fn main() -> ! {
    //uart_init();


    let disp = Display::init();

    disp.print_str("HELLO WORLD", 0xFFFFFFFF);

    loop {
        // On
        GPIO::set(29);
        Timer::delay(Duration::from_secs(2));

        // Off
        GPIO::clear(29);
        Timer::delay(Duration::from_secs(2));
    }


}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}