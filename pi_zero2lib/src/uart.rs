pub fn uart_init() {
    unsafe {
        core::ptr::write_volatile(0x3F20_0004 as *mut u32, 0b100 << 12); // GPIO14 = ALT0
        core::ptr::write_volatile(0x3F20_0004 as *mut u32, 0b100 << 15); // GPIO15 = ALT0

        // Configure UART (PL011)
        core::ptr::write_volatile(0x3F20_1000 as *mut u32, 0); // Disable UART
        core::ptr::write_volatile(0x3F20_100C as *mut u32, 1);  // Set baud rate (1 = 115200)
        core::ptr::write_volatile(0x3F20_1010 as *mut u32, 0);  // Disable interrupts
        core::ptr::write_volatile(0x3F20_1000 as *mut u32, 1 << 0 | 1 << 8 | 1 << 9); // Enable
    }
}

pub fn uart_write_byte(c: u8) {
    unsafe {
        while core::ptr::read_volatile(0x3F20_1018 as *const u32) & (1 << 5) != 0 {}
        core::ptr::write_volatile(0x3F20_1000 as *mut u32, c as u32);
    }
}

pub fn uart_write_number(mut num: u32) {
    if num == 0 {
        uart_write_byte(b'0');
    }

    let mut digits: [u8; 32] = [0; 32];
    let mut idx = 0;

    while num > 0 {
        digits[idx] = (num % 10) as u8;
        num /= 10;
        idx += 1;
    }

    let mut trailed = false;
    for digit in digits.iter().rev() {
        if trailed || *digit != 0 {
            uart_write_byte('0' as u8 + digit);
            trailed = true;
        }
    }

    uart_write_byte(b'\n');
}
