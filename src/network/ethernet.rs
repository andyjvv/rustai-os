use alloc::vec::Vec;

// Estructura para paquetes Ethernet
pub struct EthernetFrame {
    destination: [u8; 6],
    source: [u8; 6],
    ethertype: u16,
    payload: Vec<u8>,
    // No incluimos CRC/FCS por simplicidad
}

impl EthernetFrame {
    pub fn new(destination: [u8; 6], source: [u8; 6], ethertype: u16, payload: Vec<u8>) -> Self {
        EthernetFrame {
            destination,
            source,
            ethertype,
            payload,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(14 + self.payload.len());
        
        // Dirección MAC de destino
        bytes.extend_from_slice(&self.destination);
        
        // Dirección MAC de origen
        bytes.extend_from_slice(&self.source);
        
        // EtherType (tipo de protocolo)
        bytes.push((self.ethertype >> 8) as u8);
        bytes.push((self.ethertype & 0xFF) as u8);
        
        // Datos de carga útil
        bytes.extend_from_slice(&self.payload);
        
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 14 {
            return None; // Frame demasiado pequeño
        }
        
        let mut destination = [0u8; 6];
        let mut source = [0u8; 6];
        
        destination.copy_from_slice(&bytes[0..6]);
        source.copy_from_slice(&bytes[6..12]);
        
        let ethertype = ((bytes[12] as u16) << 8) | (bytes[13] as u16);
        let payload = bytes[14..].to_vec();
        
        Some(EthernetFrame {
            destination,
            source,
            ethertype,
            payload,
        })
    }
    
    pub fn get_destination(&self) -> [u8; 6] {
        self.destination
    }
    
    pub fn get_source(&self) -> [u8; 6] {
        self.source
    }
    
    pub fn get_ethertype(&self) -> u16 {
        self.ethertype
    }
    
    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
}

// Constantes para EtherType
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;