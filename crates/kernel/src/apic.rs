//! When enabled, Processor x2APIC Support helps operating systems run more efficiently on high core count configurations
//! and optimizes interrupt distribution in virtualized environments.
//!
//! Enabled mode does not enable x2APIC hardware, but provides the support necessary to the operating system.

use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x2apic::{lapic::{LocalApicBuilder, LocalApic}, ioapic::{IoApic, IrqMode, IrqFlags}};

use crate::PHYSICAL_MEM_OFFSET;

pub static LAPIC: Lazy<Mutex<LocalApic>> = Lazy::new(|| {
    let phys_addr = unsafe { x2apic::lapic::xapic_base() };
    let virt_addr = phys_addr + *PHYSICAL_MEM_OFFSET.get().unwrap();

    let lapic = LocalApicBuilder::new()
        .timer_vector(ApicInterruptIndex::Timer as usize)
        .error_vector(ApicInterruptIndex::Error as usize)
        .spurious_vector(ApicInterruptIndex::Spurious as usize)
        .set_xapic_base(virt_addr)
        .build()
        .expect("Failed to build LocalApic");

    lapic.into()
});

const IOAPIC_IRQ_OFFSET: u8 = 0x20;

pub static mut IOAPIC: Lazy<Mutex<IoApic>> = Lazy::new(|| unsafe {
    let phys_addr = 0xFEC0_0000;
    let virt_addr = phys_addr + *PHYSICAL_MEM_OFFSET.get().unwrap();

    let mut ioapic = IoApic::new(virt_addr);
    ioapic.init(IOAPIC_IRQ_OFFSET);

    let apic_id = LAPIC.lock().id();

    let mut keyboard_entry = ioapic.table_entry(IrqVector::Keyboard as u8);
    keyboard_entry.set_mode(IrqMode::Fixed);
    keyboard_entry.set_flags(IrqFlags::MASKED);
    keyboard_entry.set_dest(apic_id as u8);
    ioapic.set_table_entry(IrqVector::Keyboard as u8, keyboard_entry);

    ioapic.enable_irq(IrqVector::Keyboard as u8);

    ioapic.into()
});

pub fn init() {
    unsafe {
        disable_pic();
        LAPIC.lock().enable();
        IOAPIC.lock().init(0);
        x86_64::instructions::interrupts::enable();
    }
}


const PIC_1_OFFSET: u8 = 0x20;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

unsafe fn disable_pic() {
    let mut pics = ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET);
    pics.initialize();
    pics.disable();
}

#[repr(usize)]
pub enum ApicInterruptIndex {
    Timer = 32,
    Keyboard = 33,
    Error = 60,
    Spurious = 61,
}

#[repr(u8)]
pub enum IrqVector {
    Keyboard = 1
}
