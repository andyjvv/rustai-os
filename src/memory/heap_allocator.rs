use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Asignador de memoria con bloqueo
pub struct LockedHeapAllocator(spin::Mutex<Option<linked_list_allocator::Heap>>);

impl LockedHeapAllocator {
    /// Crea un nuevo asignador vacío
    pub const fn new() -> Self {
        LockedHeapAllocator(spin::Mutex::new(None))
    }
    
    /// Inicializa el asignador con el heap dado
    pub fn init(&self, heap_start: usize, heap_size: usize) {
        let mut heap = self.0.lock();
        let heap_space = unsafe {
            core::slice::from_raw_parts_mut(heap_start as *mut u8, heap_size)
        };
        *heap = Some(unsafe { linked_list_allocator::Heap::new(heap_space) });
    }
}

unsafe impl GlobalAlloc for LockedHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.0.lock();
        match &mut *heap {
            Some(heap) => heap.allocate_first_fit(layout)
                .ok()
                .map_or(null_mut(), |allocation| allocation.as_ptr()),
            None => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut heap = self.0.lock();
        if let Some(heap) = &mut *heap {
            heap.deallocate(core::ptr::NonNull::new_unchecked(ptr), layout);
        }
    }
}

/// Inicializa el heap del kernel
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Mapear todas las páginas del heap
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    // Inicializar el asignador
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}