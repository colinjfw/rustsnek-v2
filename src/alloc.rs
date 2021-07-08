use std::alloc;
use crate::stats::STATS;

struct CustomAlloc {}

unsafe impl alloc::GlobalAlloc for CustomAlloc {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let ret = alloc::System.alloc(layout);
        if !ret.is_null() {
            STATS.alloc_bytes.inc_by(layout.size());
            STATS.alloc_calls.inc();
        }
        return ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        alloc::System.dealloc(ptr, layout);
        STATS.dealloc_bytes.inc_by(layout.size());
        STATS.dealloc_calls.inc();
    }
}

#[global_allocator]
static ALLOCATOR: CustomAlloc = CustomAlloc{};
