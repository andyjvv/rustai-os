mod idt;
mod exceptions;
mod pic;

pub use self::idt::*;
pub use self::exceptions::*;
pub use self::pic::*;

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
