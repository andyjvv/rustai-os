pub mod paging;
pub mod frame_allocator;
pub mod heap_allocator;

pub use self::paging::*;
pub use self::frame_allocator::*;
pub use self::heap_allocator::*;

use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::PageTable;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

/// Inicializa un nuevo mapeo.
///
/// Este función es insegura porque el llamante debe garantizar que la
/// tabla de páginas activa completa está mapeada a la dirección física
/// especificada por `physical_memory_offset`. Además, esta función
/// debe ser llamada solo una vez para evitar condiciones de carrera.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> paging::OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    paging::OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Devuelve una referencia mutable a la tabla activa de nivel 4.
///
/// Esta función es insegura porque el llamante debe garantizar que la
/// tabla de páginas activa completa está mapeada a la dirección física
/// especificada por `physical_memory_offset`. Además, esta función
/// debe ser llamada solo una vez para evitar condiciones de carrera.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Convierte una dirección virtual en su dirección física mapeada.
///
/// Esta función es insegura porque el llamante debe garantizar que la
/// tabla de páginas activa completa está mapeada a la dirección física
/// especificada por `physical_memory_offset`.
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    translate_addr_inner(addr, physical_memory_offset)
}

/// Detalles de implementación privados de `translate_addr`.
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // Leer la tabla activa de nivel 4
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        paging::indexes::p4_index(addr),
        paging::indexes::p3_index(addr),
        paging::indexes::p2_index(addr),
        paging::indexes::p1_index(addr),
    ];
    let mut frame = level_4_table_frame;

    // Recorrer la tabla de páginas multinivel
    for &index in &table_indexes {
        // Convertir la dirección física del frame a una dirección virtual
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // Leer el descriptor de la tabla de páginas
        let entry = &table[index];
        // Obtener el frame del descriptor (si está presente)
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("frames enormes no soportados"),
        };
    }

    // Calcular la dirección física combinando el frame y el offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
