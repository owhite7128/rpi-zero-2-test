#![no_std]
#![no_main]

use pi_zero2lib::{ alloc::*, display::{self, Display}, gpio::{ GPIOfunc, GPIO }, time::{ Duration, Instant, Timer }, uart::* };
use core::{alloc::Layout, arch::asm, cell::UnsafeCell, default, sync::atomic::{AtomicBool, Ordering}};
extern crate alloc;
use core::alloc::GlobalAlloc;
use alloc::{boxed::Box, vec::Vec};


static IRQ_OCCURRED: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table")]
pub unsafe extern "C" fn __vector_table() {
    unsafe {asm!(
        // Current EL with SP0
        ".balign 2048",
        "b current_el_sp0_sync",
        
        ".balign 128",
        "b current_el_sp0_irq",
        
        ".balign 128",
        "b current_el_sp0_fiq",
        
        ".balign 128",
        "b current_el_sp0_serror",
        
        // Current EL with SPx
        ".balign 128",
        "b current_el_spx_sync",  // SVC goes here if at same EL
        
        ".balign 128",
        "b current_el_spx_irq",   // IRQs go here
        
        ".balign 128",
        "b current_el_spx_fiq",
        
        ".balign 128",
        "b current_el_spx_serror",
        
        // Lower EL using AArch64
        ".balign 128",
        "b lower_el_aarch64_sync",
        
        ".balign 128",
        "b lower_el_aarch64_irq",
        
        ".balign 128",
        "b lower_el_aarch64_fiq",
        
        ".balign 128",
        "b lower_el_aarch64_serror",
        
        // Lower EL using AArch32
        ".balign 128",
        "b lower_el_aarch32_sync",
        
        ".balign 128",
        "b lower_el_aarch32_irq",
        
        ".balign 128",
        "b lower_el_aarch32_fiq",
        
        ".balign 128",
        "b lower_el_aarch32_serror",
        
        options(noreturn, nostack, nomem)
    );
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_sp0_sync() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_sp0_irq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_sp0_fiq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_sp0_serror() {
    uart_write_byte(b'E');
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_spx_sync() {
    uart_write_byte(b'S');
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_spx_irq() {
    unsafe {     
        asm!(
            "msr DAIFSet, #2", // Disable Interrupts
        );
        
        handle_timer_irq();
        
        asm!(    
            "msr DAIFClr, #2",
            "eret",
        );
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_spx_fiq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn current_el_spx_serror() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch64_sync() {
    unsafe {         
        asm!(
            "msr DAIFSet, #2", // Disable Interrupts
        );


        let mut num: u64;
        asm!(
            "mov {0}, x0",
            out(reg) num
        );

        (&*DISPLAY).print_num(num as u32, 0xFFFFFFFF);


        asm!("eret", options(noreturn)); 
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch64_irq() {
    unsafe {     
        asm!(
            "msr DAIFSet, #2", // Disable Interrupts
        );
        
        handle_timer_irq();
        
        asm!(    
            "msr DAIFClr, #2",
            "eret",
        );
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch64_fiq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch64_serror() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch32_sync() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch32_irq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch32_fiq() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn lower_el_aarch32_serror() {
    unsafe { default_exception_handler(); }
}

#[unsafe(no_mangle)]
unsafe fn default_exception_handler() {
    uart_write_byte(b'I');
    loop {
        GPIO::set(29);
        Timer::delay(Duration::from_millis(100));

        // Off
        GPIO::clear(29);
        Timer::delay(Duration::from_millis(100));
    }
}


#[inline(always)]
#[unsafe(no_mangle)]
unsafe extern "C" fn handle_timer_irq() {
    
    unsafe {
        asm!("msr CNTP_CTL_EL0, {}", in(reg) 0u64);
        asm!("msr CNTP_TVAL_EL0, {}", in(reg) 208334_u64);
        asm!("msr CNTP_CTL_EL0, {}", in(reg) 1u64);
    
        SCHEDULER.context_switch();
    }
    //IRQ_OCCURRED.store(true, Ordering::SeqCst);
}

unsafe extern "C" {
    static _stack_start: usize;
    static mut __bss_start: usize;
    static mut __bss_end: usize;
}

#[repr(C, align(16))]
#[derive(Default)]
struct TaskRegisters {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    x29: u64,
    x30: u64
}


#[repr(C, align(16))]
struct Task {
    sp: u64,
    pc: u64,
    psr: u64,
    task_regs: TaskRegisters,
    tid: u64
}
const STACK_SIZE: usize = 4096;


struct Scheduler {
    tasks: UnsafeCell<Vec<Option<Task>>>,
    curr_task: UnsafeCell<Option<Task>>,
    sp: UnsafeCell<*mut u32>,
}

unsafe impl Send for Scheduler {}
unsafe impl Sync for Scheduler {}

static SCHEDULER: Scheduler = Scheduler::new();


impl Scheduler {
    pub const fn new() -> Self {
        Self {
            tasks: UnsafeCell::new(Vec::new()),
            curr_task: UnsafeCell::new(None),
            sp: UnsafeCell::new(0 as *mut u32)
        }
    }

    pub fn init(&self) {
        // Initialize the vector in a known memory location
        let vec = Vec::with_capacity(10); // Or whatever capacity you need
        unsafe { *self.tasks.get() = vec; }
    }
    pub fn add_task(&self, func: fn() -> !) {
        let stack = unsafe { 
            // Allocate stack memory
            let base = ALLOCATOR.alloc(Layout::from_size_align_unchecked(STACK_SIZE, 16)) as *mut u8;
            // Set stack pointer to TOP of the allocated memory (grows downward)
            let stack_top = base.add(STACK_SIZE) as u64;
            stack_top as u64 & !0xF
        };

        let task = Task {
            sp: stack,
            pc: func as u64,
            psr: 0x340,
            task_regs: TaskRegisters::default(),
            tid: Instant::now().to_u64()
        };

        unsafe {&mut *self.tasks.get()}.insert(0, Some(task));
    }

    pub fn start(&self) {
        unsafe {
            let vec = self.tasks.get();

            if let Some(Some(next_task)) = (&mut *vec).pop() {
                let sp = next_task.sp;
                let lr = next_task.pc;
                let psr = next_task.psr;
                let _ = (&mut *self.curr_task.get()).insert(next_task);
    
                // Set ELR and SPSR for the first task
                asm!("msr elr_el1, {}", in(reg) lr, options(nostack));
                asm!("msr spsr_el1, {}", in(reg) psr, options(nostack));
    
                // Set SP_EL0 or SP_EL1 based on SPSel
                if (psr & 1) == 0 {
                    asm!("msr sp_el0, {}", in(reg) sp, options(nostack));
                } else {
                    asm!("mov sp, {}", in(reg) sp, options(nostack));
                }
    
                // Launch task via exception return
                asm!("eret", options(noreturn));
            }
        }
    }
    
    #[inline(always)]
    pub fn context_switch(&self) {
        // Copy regs here
        let mut tmp_tr = TaskRegisters::default();
        let regs_ptr = &mut tmp_tr as *mut TaskRegisters as *mut u64;
        unsafe {asm!(
            // Set up pointer in a temporary register that we can safely use
            "mov x29, {0}",
            
            // Save all registers except x29 (used as our pointer)
            "str x0, [x29, #0]",
            "str x1, [x29, #8]",
            "str x2, [x29, #16]",
            "str x3, [x29, #24]",
            "str x4, [x29, #32]",
            "str x5, [x29, #40]",
            "str x6, [x29, #48]",
            "str x7, [x29, #56]",
            "str x8, [x29, #64]",
            "str x9, [x29, #72]",
            "str x10, [x29, #80]",
            "str x11, [x29, #88]",
            "str x12, [x29, #96]",
            "str x13, [x29, #104]",
            "str x14, [x29, #112]",
            "str x15, [x29, #120]",
            "str x16, [x29, #128]",
            "str x17, [x29, #136]",
            "str x18, [x29, #144]",
            "str x19, [x29, #152]",
            "str x20, [x29, #160]",
            "str x21, [x29, #168]",
            "str x22, [x29, #176]",
            "str x23, [x29, #184]",
            "str x24, [x29, #192]",
            "str x25, [x29, #200]",
            "str x26, [x29, #208]",
            "str x27, [x29, #216]",
            "str x28, [x29, #224]",
            "mov x28, x29",
            "mov x29, x30",  // Temporarily save x30 in x29
            "str x29, [x28, #240]", // Store x30 to its slot
            "mrs x29, elr_el1", // Grab the saved x29 from the stack
            "str x29, [x28, #232]", // Store original x29 to its slot
            
            in(reg) regs_ptr,
        );}

        let vec = self.tasks.get() ;
        if let Some(Some(next_task)) = unsafe {&mut *vec}.pop() {
            if let Some(mut curr) = unsafe { &mut *self.curr_task.get() }.take() {
                // Save current task's state
                

                unsafe {
                    // Save all general-purpose registers x0-x30, SP, PC, PSR
                    // Capture PC (ELR) and PSR (SPSR)
                    asm!(
                        "mrs {0}, elr_el1",
                        out(reg) curr.pc,
                        options(nostack)
                    );
                    asm!(
                        "mrs {0}, spsr_el1",
                        out(reg) curr.psr,
                        options(nostack)
                    );
                    
    
                    // Save SP_EL0 or SP_EL1 based on SPSel bit
                    if (curr.psr & 1) == 0 {
                        // Task was using SP_EL0 (EL1t)
                        asm!("mrs {0}, sp_el0", out(reg) curr.sp, options(nostack));
                    } else {
                        // Task was using SP_EL1 (EL1h)
                        asm!("mov {0}, sp", out(reg) curr.sp, options(nostack));
                    }

                    (&mut *vec).insert(0, Some(curr));
                    let first = (&mut *vec)[0].as_mut().unwrap();
                    first.task_regs.x0  = tmp_tr.x0;
                    first.task_regs.x1  = tmp_tr.x1;
                    first.task_regs.x2  = tmp_tr.x2;
                    first.task_regs.x3  = tmp_tr.x3;
                    first.task_regs.x4  = tmp_tr.x4;
                    first.task_regs.x5  = tmp_tr.x5;
                    first.task_regs.x6  = tmp_tr.x6;
                    first.task_regs.x7  = tmp_tr.x7;
                    first.task_regs.x8  = tmp_tr.x8;
                    first.task_regs.x9  = tmp_tr.x9;
                    first.task_regs.x10 = tmp_tr.x10;
                    first.task_regs.x11 = tmp_tr.x11;
                    first.task_regs.x12 = tmp_tr.x12;
                    first.task_regs.x13 = tmp_tr.x13;
                    first.task_regs.x14 = tmp_tr.x14;
                    first.task_regs.x15 = tmp_tr.x15;
                    first.task_regs.x16 = tmp_tr.x16;
                    first.task_regs.x17 = tmp_tr.x17;
                    first.task_regs.x18 = tmp_tr.x18;
                    first.task_regs.x19 = tmp_tr.x19;
                    first.task_regs.x20 = tmp_tr.x20;
                    first.task_regs.x21 = tmp_tr.x21;
                    first.task_regs.x22 = tmp_tr.x22;
                    first.task_regs.x23 = tmp_tr.x23;
                    first.task_regs.x24 = tmp_tr.x24;
                    first.task_regs.x25 = tmp_tr.x25;
                    first.task_regs.x26 = tmp_tr.x26;
                    first.task_regs.x27 = tmp_tr.x27;
                    first.task_regs.x28 = tmp_tr.x28;
                    first.task_regs.x29 = tmp_tr.x29;
                    first.task_regs.x30 = tmp_tr.x30;

                    
                }

    
                // Restore next task's state
                let sp = next_task.sp & (!0xF as u64);
                let lr = next_task.pc;
                let psr = next_task.psr;

                // Update task lists
                let curr_task_mut = unsafe { &mut *self.curr_task.get() }.insert(next_task);

                unsafe {
                    // Set ELR and SPSR for next task
                    asm!("msr elr_el1, {}", in(reg) lr, options(nomem, nostack));
                    asm!("msr spsr_el1, {}", in(reg) psr, options(nomem, nostack));
    
                    // Restore SP_EL0 or SP_EL1 based on SPSel
                    if (psr & 1) == 0 {
                        // Next task uses SP_EL0 (EL1t)
                        asm!("msr sp_el0, {}", in(reg) sp, options(nomem, nostack));
                    } else {
                        // Next task uses SP_EL1 (EL1h)
                        asm!("mov sp, {}", in(reg) sp, options(nomem, nostack));
                    }
                }

                unsafe {
                    let regs_ptr = &mut (curr_task_mut.task_regs) as *mut TaskRegisters as *mut u64;
                
                
                    asm!(
                        // Set up pointer in x29 (we'll restore actual x29 last)
                        "mov x29, {0}",
                        
                        // Restore all registers except x29 and x30
                        "ldr x0, [x29, #0]",
                        "ldr x1, [x29, #8]",
                        "ldr x2, [x29, #16]",
                        "ldr x3, [x29, #24]",
                        "ldr x4, [x29, #32]",
                        "ldr x5, [x29, #40]",
                        "ldr x6, [x29, #48]",
                        "ldr x7, [x29, #56]",
                        "ldr x8, [x29, #64]",
                        "ldr x9, [x29, #72]",
                        "ldr x10, [x29, #80]",
                        "ldr x11, [x29, #88]",
                        "ldr x12, [x29, #96]",
                        "ldr x13, [x29, #104]",
                        "ldr x14, [x29, #112]",
                        "ldr x15, [x29, #120]",
                        "ldr x16, [x29, #128]",
                        "ldr x17, [x29, #136]",
                        "ldr x18, [x29, #144]",
                        "ldr x19, [x29, #152]",
                        "ldr x20, [x29, #160]",
                        "ldr x21, [x29, #168]",
                        "ldr x22, [x29, #176]",
                        "ldr x23, [x29, #184]",
                        "ldr x24, [x29, #192]",
                        "ldr x25, [x29, #200]",
                        "ldr x26, [x29, #208]",
                        "ldr x27, [x29, #216]",
                        "ldr x28, [x29, #224]",
                        
                        // Save pointer register temporarily
                        "mov x27, x29",
                        
                        // Restore x29 (frame pointer)
                        "ldr x29, [x27, #232]",
                        
                        // Restore x30 (link register)
                        "ldr x30, [x27, #240]",
                        
                        in(reg) regs_ptr,
                    );}
            }
        }
    }
}


#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov x0, (1 << 31)",      // EL1 uses aarch64
            "msr hcr_el2, x0",
            "mov x0, 0x3c5",          // EL1h (SPSel = 1) with interrupt disabled
            "msr spsr_el2, x0",
            "adr x0, 1f",             // Load address of label 1 into x0
            "msr elr_el2, x0",        // Set elr_el2 to the instruction after eret
            "mov x1, #(3 << 20)",     // Enable SIMD/FP (bits 20-21 set to 0b11)
            "msr cpacr_el1, x1",      // Set CPACR_EL1 to enable SIMD/FP at EL1
            "eret",                   
            "1:",                     // This label marks the instruction after eret
        );

        asm!(
            "ldr x0, =_stack_start",  // Load the address of _stack_start into x0
            "mov sp, x0",             // Set the stack pointer (sp) to the value in x0
            options(nostack, nomem)
        );

        asm!(
            "adr x0, .vector_table",
            "msr vbar_el1, x0",
            "isb",  
            options(nostack, nomem, preserves_flags)
        );

        asm!(
            "msr DAIFSet, #2",
            options(nostack, nomem, preserves_flags)
        );

        asm!("msr CNTP_CTL_EL0, {}", in(reg) 0u64, options(nostack, nomem, preserves_flags));
        
        asm!("msr CNTP_TVAL_EL0, {}", in(reg) 208334_u64, options(nostack, nomem, preserves_flags));
        
        // Enable the timer by writing 1 to the control register
        asm!("msr CNTP_CTL_EL0, {}", in(reg) 1u64, options(nostack, nomem, preserves_flags));

        core::ptr::write_volatile((0x4000_0040) as *mut u32, 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3);
        
        
        main();
    }
}

static mut DISPLAY: *mut Display = 0 as *mut Display;

fn task1() -> ! {
    loop {
        unsafe {
            asm!(
                "mov x0, #1",
                "svc #1"
            );

        }
        Timer::delay(Duration::from_secs(5));
    }
}
fn task2() -> ! {
    loop {
        unsafe {
            asm!(
                "mov x0, #2",
                "svc #1"
            );

        }
        Timer::delay(Duration::from_secs(5));
    }
}
fn task3() -> ! {
    loop {
        unsafe {
            asm!(
                "mov x0, #3",
                "svc #1"
            );

        }
        Timer::delay(Duration::from_secs(5));
    }
}



#[allow(static_mut_refs)]
fn main() -> ! {
    uart_init();

    GPIO::fsel(29, GPIOfunc::OUT);
    let mut disp = Box::new(Display::init());
    unsafe {DISPLAY = &mut *disp}
    SCHEDULER.init();

    SCHEDULER.add_task(task1);
    SCHEDULER.add_task(task2);
    SCHEDULER.add_task(task3);

    SCHEDULER.start();

    loop {
        if IRQ_OCCURRED.load(Ordering::SeqCst) {
            // Handle the IRQ effects here in main context
            disp.print_str("IRQ\n", 0xFFFFFFFF);
            // Reset the flag
            IRQ_OCCURRED.store(false, Ordering::SeqCst);
        }

        // On
        /*GPIO::set(29);
        Timer::delay(Duration::from_secs(1));

        // Off
        GPIO::clear(29);
        Timer::delay(Duration::from_secs(1));*/
    }


}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}