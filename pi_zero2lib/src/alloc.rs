use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

const ARENA_SIZE: usize = 256 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] 
pub struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
}

#[global_allocator]
pub static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(Relaxed, Relaxed, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        unsafe { self.arena.get().cast::<u8>().add(allocated) }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
