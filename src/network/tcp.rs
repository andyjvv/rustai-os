use alloc::vec::Vec;
use core::fmt;

// Cabecera TCP simplificada
#[derive(Debug, Clone)]
pub struct TcpHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    data_offset: u8,    // En palabras de 32 bits (4 bytes)
    flags: TcpFlags,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    // No incluimos opciones por simplicidad
}

#[derive(Debug, Clone, Copy)]
pub struct TcpFlags {
    fin: bool,
    syn: bool,
    rst: bool,
    psh: bool,
    ack: bool,
    urg: bool,
}

impl TcpHeader {
    pub fn new(
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: TcpFlags,
        window_size: u16
    ) -> Self {
        TcpHeader {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            data_offset: 5, // 5 palabras de 32 bits = 20 bytes (sin opciones)
            flags,
            window_size,
            checksum: 0,    // Se calcula después
            urgent_pointer: 0,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20); // Tamaño mínimo sin opciones
        
        // Puerto origen (2 bytes)
        bytes.push((self.source_port >> 8) as u8);
        bytes.push((self.source_port & 0xFF) as u8);
        
        // Puerto destino (2 bytes)
        bytes.push((self.destination_port >> 8) as u8);
        bytes.push((self.destination_port & 0xFF) as u8);
        
        // Número de secuencia (4 bytes)
        bytes.push((self.sequence_number >> 24) as u8);
        bytes.push((self.sequence_number >> 16) as u8);
        bytes.push((self.sequence_number >> 8) as u8);
        bytes.push((self.sequence_number & 0xFF) as u8);
        
        // Número de confirmación (4 bytes)
        bytes.push((self.acknowledgment_number >> 24) as u8);
        bytes.push((self.acknowledgment_number >> 16) as u8);
        bytes.push((self.acknowledgment_number >> 8) as u8);
        bytes.push((self.acknowledgment_number & 0xFF) as u8);
        
        // Data offset y flags (2 bytes)
        let offset_and_reserved = (self.data_offset << 4) & 0xF0;
        let flags_byte1 = if self.flags.urg { 0x20 } else { 0 } |
                          if self.flags.ack { 0x10 } else { 0 } |
                          if self.flags.psh { 0x08 } else { 0 } |
                          if self.flags.rst { 0x04 } else { 0 } |
                          if self.flags.syn { 0x02 } else { 0 } |
                          if self.flags.fin { 0x01 } else { 0 };
        
        bytes.push(offset_and_reserved);
        bytes.push(flags_byte1);
        
        // Window size (2 bytes)
        bytes.push((self.window_size >> 8) as u8);
        bytes.push((self.window_size & 0xFF) as u8);
        
        // Checksum (2 bytes)
        bytes.push((self.checksum >> 8) as u8);
        bytes.push((self.checksum & 0xFF) as u8);
        
        // Urgent pointer (2 bytes)
        bytes.push((self.urgent_pointer >> 8) as u8);
        bytes.push((self.urgent_pointer & 0xFF) as u8);
        
        bytes
    }
}

impl fmt::Display for TcpFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags = Vec::new();
        
        if self.fin { flags.push("FIN"); }
        if self.syn { flags.push("SYN"); }
        if self.rst { flags.push("RST"); }
        if self.psh { flags.push("PSH"); }
        if self.ack { flags.push("ACK"); }
        if self.urg { flags.push("URG"); }
        
        write!(f, "{}", flags.join("|"))
    }
}