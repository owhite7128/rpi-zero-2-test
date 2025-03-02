use core::{cell::Cell, str};

use super::font::*;


#[repr(C, align(16))]
struct MailboxBuffer {
    size: u32,
    request_code: u32,   // 0 = request
    tag0_id: u32,        // 0x00048003 (set physical size)
    tag0_size: u32,      // 8 bytes (2 words)
    tag0_code: u32,      // 0 (request)
    width: u32,          // e.g., 1024
    height: u32,         // e.g., 768
    tag1_id: u32,
    tag1_size: u32,
    tag1_code: u32,
    vwidth: u32,
    vheight: u32,
    tag2_id: u32,
    tag2_size: u32,
    tag2_code: u32,
    depth: u32,
    tag3_id: u32,        // 0x00040001 (allocate buffer)
    tag3_size: u32,      // 8 bytes (2 words)
    tag3_code: u32,      // 0 (request)
    fb_addr: u32,        // QEMU fills this in
    fb_size: u32,        // QEMU fills this in
    tag4_id: u32,
    tag4_size: u32,
    tag4_code: u32,
    pitch: u32,
    end_tag: u32,        // 0 (terminator)
}

impl Default for MailboxBuffer {
    fn default() -> Self {
        Self {
            size: (core::mem::size_of::<Self>() - 4) as u32, // Exclude `end_tag`
            request_code: 0,
            tag0_id: 0x00048003, //phy size
            tag0_size: 8,
            tag0_code: 8,
            width: 640,
            height: 480,
            tag1_id: 0x00048004, //v size
            tag1_size: 8,
            tag1_code: 8,
            vwidth: 640,
            vheight: 480,
            tag2_id: 0x00048005, // s_depth
            tag2_size: 4,
            tag2_code: 4,
            depth: 32,
            tag3_id: 0x00040001,
            tag3_size: 8,
            tag3_code: 4,
            fb_addr: 16,
            fb_size: 0,
            tag4_id: 0x00040008,
            tag4_size: 4,
            tag4_code: 0,
            pitch: 0,
            end_tag: 0,
        }
    }
}

#[inline(never)]
fn mailbox_write(buffer: &mut MailboxBuffer) {
    let addr = buffer as *mut MailboxBuffer as u32;
    let mailbox = 0x3F00_B880 as *mut u32;

    unsafe {
        // Combine buffer address (upper 28 bits) and channel 8 (lower 4 bits)
        let data = (addr & !0xF) | 8;

        // Wait until mailbox is ready to write
        while core::ptr::read_volatile(mailbox.add(6)) & 0x80000000 != 0 {}
        // Write to mailbox
        core::ptr::write_volatile(mailbox.add(8), data);
    }
}

#[inline(never)]
fn mailbox_read(buffer: &mut MailboxBuffer) -> bool {
    let mailbox = 0x3F00_B880 as *mut u32;

    unsafe {
        // Wait until mailbox has data
        while core::ptr::read_volatile(mailbox.add(6)) & 0x40000000 != 0 {}
        // Read data (must check channel)
        let data = core::ptr::read_volatile(mailbox.add(0));
        if (data & 0xF) != 8 {
            return false; // Wrong channel
        }

        // Verify response (success = 0x80000000)
        buffer.request_code == 0x80000000
    }
}


#[derive(Clone, Copy)]
struct Offset {
    x_off: u32,
    y_off: u32,
}


pub struct Display {
    addr: *mut u8,
    width: u32,
    height: u32,
    vwidth: u32,
    vheight: u32,
    bpp: u32,
    pitch: u32,
    fb_size: u32,
    offset_cell: Cell<Offset>
}

impl Display {
    pub const fn empty() -> Self {
        Self {
            addr: 0 as *mut u8,
            width: 0,
            height: 0,
            vwidth: 0,
            vheight: 0,
            bpp: 0,
            pitch: 0,
            fb_size: 0,
            offset_cell: Cell::new(Offset {
                x_off: 0,
                y_off: 0
            })
        }

    }
    pub fn init() -> Self {
        let mut mb = MailboxBuffer::default();
        mailbox_write(&mut mb);

        while mailbox_read(&mut mb) && mb.tag3_code != 0x80000008 && mb.fb_size == 0  {};
        let fb_addr = ((mb.fb_addr | 0x40000000) & !0xC0000000) as *mut u8;

        Self {
            addr: fb_addr,
            width: mb.width,
            height: mb.height,
            vwidth: mb.vwidth,
            vheight: mb.vheight,
            bpp: mb.depth,
            pitch: mb.pitch,
            fb_size: mb.fb_size,
            offset_cell: Cell::new(Offset {x_off: 0, y_off: 8})
        }
    }

    pub fn draw_pixel(&self, x: u32, y: u32, bgra: u32) {
        let mut pixel_offset = (x * (32 >> 3)) + (y * (self.pitch));
        if pixel_offset < self.fb_size {
            

            if self.bpp == 32 {
                unsafe { self.addr.add(pixel_offset as usize).write_volatile(((bgra & 0xFF000000) >> 24) as u8) };
                pixel_offset += 1;
                unsafe { self.addr.add(pixel_offset as usize).write_volatile(((bgra & 0xFF0000) >> 16) as u8) };
                pixel_offset += 1;
                unsafe { self.addr.add(pixel_offset as usize).write_volatile(((bgra & 0xFF00) >> 8) as u8) };
                pixel_offset += 1;
                unsafe { self.addr.add(pixel_offset as usize).write_volatile((bgra & 0xFF) as u8) };
            } else {
                unsafe { self.addr.add(pixel_offset as usize).write_volatile(((bgra & 0xFF00) >> 8) as u8) };
                pixel_offset += 1;
                unsafe { self.addr.add(pixel_offset as usize).write_volatile((bgra & 0xFF) as u8) };
            }

        }
    }

    pub fn draw_a(&self, x_off: u32, y_off: u32, bgra: u32) {
        let a: [bool; 64] = [false, false, true, true, true, false, false, false,
                            false, true, true, false, true, true, false, false,
                            true, true, false, false, false, true, true, false,
                            true, true, false, false, false, true, true, false,
                            true, true, true, true, true, true, true, false,
                            true, true, false, false, false, true, true, false,
                            true, true, false, false, false, true, true, false,
                            true, true, false, false, false, true, true, false];

        for (idx, val) in a.iter().enumerate() {
            if *val {
                let x = (idx % 8) as u32;
                let y = (idx / 8) as u32;
                self.draw_pixel(x + x_off, y + y_off, bgra);

            }
        }
    }

    pub fn print_char(&self, ch: char, bgra: u32) {
        let mut nl = 0;
        let letter = if ch as u8 >= 65 && ch as u8 <= 90 {
            let idx = (ch as u8 - b'A') as usize;
            if idx > 25 {
                ALPHABET[0]
            } else {
                ALPHABET[idx]
            }
        } else if ch as u8 >= 48 && ch as u8 <= 57 {
            let idx = (ch as u8 - b'0') as usize;
            if idx > 25 {
                NUMBERS[0]
            } else {
                NUMBERS[idx]
            }
        } else if ch as u8 == 10 {
            nl = 1;
            SPECIAL_CHARS[0]
        } else {
            SPECIAL_CHARS[0]
        };

        let offset = self.offset_cell.get();

        let (curr_x, curr_y) = if (offset.x_off + 8) >= self.width {
            (0, offset.y_off + 8)
        } else {
            (offset.x_off, offset.y_off)
        };

        for (idx, val) in letter.iter().enumerate() {
            if *val {
                let x = (idx % 8) as u32;
                let y = (idx / 8) as u32;
                self.draw_pixel(x + curr_x, y + curr_y, bgra);

            }
        }
        let (new_x_off, new_y_off) = if nl == 1 {
            (0, curr_y + 8)
        } else {
            (curr_x + 8, curr_y)
        };

        self.offset_cell.set(Offset {
            x_off: new_x_off,
            y_off: new_y_off
        })
    } 

    pub fn print_str(&self, string: &str, bgra: u32) {
        for ch in string.chars() {
            self.print_char(ch, bgra);
        }

    }

    pub fn print_num(&self, mut num: u32, bgra: u32) {
        if num == 0 {
            self.print_char('0', bgra);
            return
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
                self.print_char(('0' as u8 + digit) as char, bgra);
                trailed = true;
            }
        }
    }
}

unsafe impl Sync for Display {}

unsafe impl Send for Display {}