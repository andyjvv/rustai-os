/// Devuelve un iterador sobre los frames disponibles descritos por
    /// el mapa de memoria.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Obtener regiones usables del mapa de memoria
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        
        // Convertir cada región en un iterador de direcciones de frames
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        
        // Transformar direcciones en frame PhysFrame
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        
        // Crear un PhysFrame para cada dirección
        frame_addresses.map(|addr| {
            PhysFrame::containing_address(PhysAddr::new(addr))
        })
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames()
            .nth(self.next);
        self.next += 1;
        frame
    }
}