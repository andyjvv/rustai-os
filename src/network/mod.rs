mod ethernet;
mod tcp;
mod protocols;

use crate::println;
use lazy_static::lazy_static;
use spin::Mutex;

pub use self::ethernet::*;
pub use self::tcp::*;
pub use self::protocols::*;

pub struct NetworkSubsystem {
    initialized: bool,
    // Configuración del subsistema de red
    interfaces: Vec<NetworkInterface>,
}

pub struct NetworkInterface {
    name: String,
    mac_address: [u8; 6],
    ip_address: Option<[u8; 4]>,
    status: InterfaceStatus,
}

pub enum InterfaceStatus {
    Down,
    Up,
    ConfiguringDHCP,
    Ready,
}

impl NetworkSubsystem {
    pub fn new() -> Self {
        NetworkSubsystem {
            initialized: false,
            interfaces: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Detectar interfaces de red
        let detected_interfaces = detect_network_interfaces();
        self.interfaces = detected_interfaces;
        
        self.initialized = true;
        
        println!("Subsistema de Red inicializado");
        println!("  - Interfaces detectadas: {}", self.interfaces.len());
        
        // Mostrar información de interfaces
        for (i, interface) in self.interfaces.iter().enumerate() {
            println!("  Interface {}: {} ({:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X})",
                i, interface.name,
                interface.mac_address[0], interface.mac_address[1],
                interface.mac_address[2], interface.mac_address[3],
                interface.mac_address[4], interface.mac_address[5]);
        }
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    pub fn get_interfaces(&self) -> &[NetworkInterface] {
        &self.interfaces
    }
}

lazy_static! {
    static ref NETWORK_SUBSYSTEM: Mutex<NetworkSubsystem> = Mutex::new(NetworkSubsystem::new());
}

pub fn init() {
    NETWORK_SUBSYSTEM.lock().initialize();
}

// Función para demostración
fn detect_network_interfaces() -> Vec<NetworkInterface> {
    // En un sistema real, esto detectaría el hardware de red disponible
    let mut interfaces = Vec::new();
    
    // Agregar una interfaz ficticia para demostración
    interfaces.push(NetworkInterface {
        name: String::from("eth0"),
        mac_address: [0x52, 0x55, 0x53, 0x54, 0x41, 0x49],
        ip_address: None,
        status: InterfaceStatus::Down,
    });
    
    interfaces
}