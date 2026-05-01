#![no_std]

use axallocator::{AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

/// Early memory allocator.
///
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward (from `start`)
/// - Alloc pages backward (from `end`)
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    alloc_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            alloc_count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = self.end;
        self.alloc_count = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        // Single region, already initialized.
        Ok(())
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();

        // Align b_pos up to the required alignment
        let addr = (self.b_pos + align - 1) & !(align - 1);

        // Check if there's enough space (b_pos grows forward, p_pos grows backward)
        if addr + size > self.p_pos {
            return Err(axallocator::AllocError::NoMemory);
        }

        let ptr = NonNull::new(addr as *mut u8).unwrap();
        self.b_pos = addr + size;
        self.alloc_count += 1;
        Ok(ptr)
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        self.alloc_count = self.alloc_count.saturating_sub(1);
        if self.alloc_count == 0 {
            // Reset b_pos only when all byte allocations are freed.
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos.saturating_sub(self.b_pos)
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        self.alloc_pages_at(0, num_pages, align_pow2)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        // Pages are never freed in this bump allocator.
    }

    fn alloc_pages_at(
        &mut self,
        _base: usize,
        num_pages: usize,
        align_pow2: usize,
    ) -> AllocResult<usize> {
        let align = align_pow2.max(PAGE_SIZE);
        let size = num_pages * PAGE_SIZE;

        // Align p_pos down to the required alignment (p_pos grows backward)
        let addr = (self.p_pos - size) & !(align - 1);

        // Check if there's enough space
        if addr < self.b_pos {
            return Err(axallocator::AllocError::NoMemory);
        }

        self.p_pos = addr;
        Ok(addr)
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / PAGE_SIZE
    }
}
