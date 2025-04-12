#![no_std] // No usar la biblioteca estándar de Rust
#![no_main] // No usar el punto de entrada estándar de Rust
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod vga_buffer;
mod gdt;
mod interrupts;
mod memory;
mod allocator;
mod ai;
mod network;
mod api;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use alloc::string::String;

#[global_allocator]
static ALLOCATOR: allocator::LockedHeapAllocator = allocator::LockedHeapAllocator::new();

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("RustAI-OS: Inicializando...");
    
    // Inicializar componentes del sistema
    gdt::init();
    interrupts::init();
    
    // Configurar gestor de memoria
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // Inicializar asignador de memoria
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Fallo en la inicialización del heap");
    
    // Inicializar subsistemas
    ai::init();
    network::init();
    api::init();
    
    println!("RustAI-OS: Sistema listo para operaciones de IA");
    
    #[cfg(test)]
    test_main();
    
    // Bucle principal
    loop {
        x86_64::instructions::hlt();
    }
}

/// Controlador de pánico del kernel
/// 
/// Este controlador se ejecuta cuando ocurre un pánico en el kernel.
/// Proporciona información detallada sobre el error, incluyendo ubicación
/// y mensaje, además de realizar un apagado seguro de los componentes críticos.
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    
    // Deshabilitar interrupciones para evitar condiciones de carrera
    interrupts::disable();
    
    let mut serial = serial_port::SerialPort::new();
    let _ = serial.write_str("\n\n==========================================\n");
    let _ = serial.write_str("              KERNEL PANIC\n");
    let _ = serial.write_str("==========================================\n\n");
    
    // Registrar información detallada sobre el pánico
    if let Some(location) = info.location() {
        let _ = writeln!(
            serial,
            "Pánico en archivo '{}' línea {}, columna {}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        let _ = serial.write_str("Pánico sin información de ubicación\n");
    }
    
    // Imprimir el mensaje de pánico si está disponible
    if let Some(message) = info.message() {
        let _ = writeln!(serial, "Mensaje: {}", message);
    }
    
    // También mostrar en pantalla si está disponible
    if let Some(mut writer) = vga_buffer::WRITER.lock().as_mut() {
        writer.set_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::White, 
            vga_buffer::Color::Red
        ));
        writeln!(writer, "\n\nKERNEL PANIC: {}", info).unwrap();
    }
    
    // Registrar el backtrace si está disponible (función que requiere características específicas)
    #[cfg(feature = "stack-trace")]
    {
        let _ = serial.write_str("\nBacktrace:\n");
        // Implementación de backtrace omitida por brevedad
    }
    
    let _ = serial.write_str("\nSistema detenido.\n");
    
    // Intentar realizar un apagado seguro de componentes críticos
    unsafe {
        // Aquí irían llamadas a funciones de apagado seguro
        // de componentes críticos del sistema
    }
    
    // Bucle infinito final con instrucción de halt
    let _ = serial.write_str("\nEntrando en bucle infinito de halt...\n");
    loop {
        x86_64::instructions::hlt();
    }
}

// Manejador de errores de asignación
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Error de asignación: {:?}", layout)
}

// Funciones para pruebas
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Ejecutando {} pruebas", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
  
// La ubicación del módulo vga_buffer es src/vga_buffer.rs


use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Caracteres ASCII imprimibles y salto de línea
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Caracteres no ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}