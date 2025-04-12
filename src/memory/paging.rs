use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PhysFrame, Size4KiB, 
        OffsetPageTable as x86_64OffsetPageTable, 
        page_table::FrameError,
    },
};

pub use x86_64::structures::paging::{
    PageTableFlags as Flags,
    mapper::MapToError,
};

/// Alias que exportamos para mayor legibilidad
pub type OffsetPageTable<'a> = x86_64OffsetPageTable<'a>;

/// Espacio de nombres para funciones de índice
pub mod indexes {
    use x86_64::VirtAddr;
    
    pub const fn p4_index(addr: VirtAddr) -> usize {
        (addr.as_u64() >> 39) & 0o777
    }
    
    pub const fn p3_index(addr: VirtAddr) -> usize {
        (addr.as_u64() >> 30) & 0o777
    }
    
    pub const fn p2_index(addr: VirtAddr) -> usize {
        (addr.as_u64() >> 21) & 0o777
    }
    
    pub const fn p1_index(addr: VirtAddr) -> usize {
        (addr.as_u64() >> 12) & 0o777
    }
}

/// Crea un mapeo para la página especificada al frame indicado
/// en la tabla de páginas activa.
///
/// Esto es principalmente un envoltorio seguro para `map_to`. 
pub fn create_mapping(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: Flags,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)?.flush()
    };
    Ok(())
}