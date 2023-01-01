use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{gdt, apic::ApicInterruptIndex};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(handlers::breakpoint);
    idt.page_fault.set_handler_fn(handlers::page_fault);
    idt.general_protection_fault.set_handler_fn(handlers::general_protection);

    unsafe {
        idt.double_fault.set_handler_fn(handlers::double_fault)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt[ApicInterruptIndex::Timer as usize].set_handler_fn(handlers::timer);
    idt[ApicInterruptIndex::Keyboard as usize].set_handler_fn(handlers::keyboard);
    idt[ApicInterruptIndex::Mouse as usize].set_handler_fn(handlers::mouse);
    idt[ApicInterruptIndex::Error as usize].set_handler_fn(handlers::error);
    idt[ApicInterruptIndex::Spurious as usize].set_handler_fn(handlers::spurious);

    // https://github.com/redox-os/kernel/blob/ee6c9f402009ffaa43286437c09f8c1401b56e1f/src/arch/x86_64/idt.rs#L221

    idt
});

pub fn init() {
    unsafe {
        crate::gdt::init();
    }

    // TODO: Move. Mouse Initiation.
    {
        use crate::ps2::*;

        if let Err(e) = set_mouse_id(MouseId::Four) {
            println!("Failed to set the mouse id to four: {e}");
            // panic!("Failed to set the mouse id to four");
        }

        // Read it back to check that it worked.
        match mouse_id() {
            Ok(id) =>  {
                println!("The PS/2 mouse ID is: {id:?}");

                if !matches!(id, MouseId::Four) {
                    println!("Failed to set the mouse id to four");
                }
            }

            Err(e) => {
                println!("Failed to read the PS/2 mouse ID: {e}");
            }
        }
    }

    IDT.load();
}

mod handlers {
    use x86_64::{structures::idt::{InterruptStackFrame, PageFaultErrorCode}, instructions::port::Port};

    use crate::{apic::LAPIC, hlt_loop, display::framebuffer::FB_WRITER, ps2};

    pub extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
        crate::serial_println!("EXCEPTION: BREAKPOINT");
        crate::serial_println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn timer(_: InterruptStackFrame) {
        // print!(".");

        if let Some(writer) = FB_WRITER.get() {
            writer.lock().tick();
        }

        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn keyboard(_: InterruptStackFrame) {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        // TODO: DETERMINE if quick key combinations can STILL prevent key up codes from activating.
        crate::task::keyboard::add_scancode(scancode);

        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn mouse(_: InterruptStackFrame) {
        let _packet = ps2::read_mouse_packet(&ps2::MouseId::Four);

        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn error(stack_frame: InterruptStackFrame) {
        crate::serial_println!("RECEIVED ERROR INTERRUPT: {stack_frame:#?}");
        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn spurious(stack_frame: InterruptStackFrame) {
        crate::serial_println!("RECEIVED SPURIOUS INTERRUPT: {stack_frame:#?}");
        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
        crate::serial_println!("PAGE_FAULT:");
        crate::serial_println!("error code {error_code:?}");
        crate::serial_println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn general_protection(_stack_frame: InterruptStackFrame, error_code: u64) {
        crate::serial_println!("GENERAL PROTECTION FAULT:");

        if error_code > 0 {
            let ssi = error_code;

            crate::serial_println!("  Segment Selector:");
            crate::serial_println!("    External: {}", if ssi & 1 == 1 { "yes" } else { "no" });
            crate::serial_println!("    Table: {}",
                match (ssi & 0b110) >> 1 {
                    0b00 => "GDT",
                    0b01 => "IDT",
                    0b10 => "LDT",
                    0b11 => "IDT",
                    _ => unreachable!()
                }
            );
            crate::serial_println!("    Index: {}", (ssi & 0b1_1111_1111_1111) >> 3);
        } else {
            crate::serial_println!("error code {error_code:?}");
        }

        hlt_loop();

        // println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
        crate::serial_println!("DOUBLE FAULT:");
        crate::serial_println!("error code {error_code:?}");
        crate::serial_println!("{stack_frame:#?}");

        panic!("double fault");
    }
}
