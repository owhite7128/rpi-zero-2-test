

// Can just int div(10) for FSEL
// n0-n9 :
// p0 : 2-0 - <<0
// p1 : 5-3 - <<3
// p2 : 8-6 - <<6
// p3 : 11-9 - <<9
// p4 : 14-12 - <<12
// p5 : 17-15 - <<15
// p6 : 20-18 - <<18
// p7 : 23-21 - <<21
// p8 : 26-24 - <<24
// p9 : 29-27 - <<27

//  0-9  - FSEL0 - 0x7E20_0000
// 10-19 - FSEL1 - 0x7E20_0004
// 20-29 - FSEL2 - 0x7E20_0008
// 30-39 - FSEL3 - 0x7E20_000C
// 40-49 - FSEL4 - 0x7E20_0010
// 50-53 - FSEL5 - 0x7E20_0014

pub enum GPIOfunc {
    OUT,
    IN,
    /*ALT0,
    ALT1,
    ALT2,
    ALT3,
    ALT4,
    ALT5*/
}

pub struct GPIO {}

impl GPIO {
    pub fn fsel(pin: u32, func: GPIOfunc) {
        let fsel_reg_off = (pin/10) * 0x4;
        let fsel_pin_off = (pin % 10) * 0x3;
        unsafe {
            match func {
                GPIOfunc::OUT => {

                    core::ptr::write_volatile((0x3F20_0000 + fsel_reg_off) as *mut u32, 0x001 << fsel_pin_off);
                },
                GPIOfunc::IN => {
                    core::ptr::write_volatile((0x3F20_0000 + fsel_reg_off) as *mut u32, 0x000 << fsel_pin_off);
                }
            }
        }
    }

    pub fn set(pin: u32) {
        if pin > 31 {
            unsafe { core::ptr::write_volatile(0x3F20_002C  as *mut u32, 1 << (pin - 32)); }
        } else {
            unsafe { core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1 << pin); }
        }
    }
    pub fn clear(pin: u32) {
        if pin > 31 {
            unsafe { core::ptr::write_volatile(0x3F20_0020 as *mut u32, 1 << (pin - 32)); }
        } else {
            unsafe { core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << pin); }
        }
    }
}

