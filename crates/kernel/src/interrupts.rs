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
    idt[ApicInterruptIndex::Error as usize].set_handler_fn(handlers::error);
    idt[ApicInterruptIndex::Spurious as usize].set_handler_fn(handlers::spurious);

    idt
});

pub fn init() {
    unsafe {
        crate::gdt::init();
    }

    IDT.load();
}

mod handlers {
    use keyboard::{KeyEvent, KeyCode, ExtendedKeyCode};
    use x86_64::{structures::idt::{InterruptStackFrame, PageFaultErrorCode}, instructions::port::Port};

    use crate::{println, apic::LAPIC, hlt_loop, print};

    pub extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
        println!("EXCEPTION: BREAKPOINT");
        println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn timer(_: InterruptStackFrame) {
        // print!(".");
        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn keyboard(_: InterruptStackFrame) {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        // TODO: Quick key combinations can prevent key up codes from activating.

        // println!("- {scancode}");

        if let Some(KeyEvent::Down(key)) = keyboard::handle_next_scan_code(scancode) {
            match key.code {
                KeyCode::Unknown(v) => print!("[{v}]"),
                KeyCode::Extended(ExtendedKeyCode::Unknown(v)) => print!("[e{v}]"),

                _ => print!("{}", key.char),
            }
        }

        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn error(stack_frame: InterruptStackFrame) {
        println!("RECEIVED ERROR INTERRUPT: {stack_frame:#?}");
        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn spurious(stack_frame: InterruptStackFrame) {
        println!("RECEIVED SPURIOUS INTERRUPT: {stack_frame:#?}");
        unsafe { LAPIC.lock().end_of_interrupt() }
    }

    pub extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
        println!("PAGE_FAULT:");
        println!("error code {error_code:?}");
        println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn general_protection(_stack_frame: InterruptStackFrame, error_code: u64) {
        println!("GENERAL PROTECTION FAULT:");

        if error_code > 0 {
            let ssi = error_code;

            println!("  Segment Selector:");
            println!("    External: {}", if ssi & 1 == 1 { "yes" } else { "no" });
            println!("    Table: {}",
                match (ssi & 0b110) >> 1 {
                    0b00 => "GDT",
                    0b01 => "IDT",
                    0b10 => "LDT",
                    0b11 => "IDT",
                    _ => unreachable!()
                }
            );
            println!("    Index: {}", (ssi & 0b1_1111_1111_1111) >> 3);
        } else {
            println!("error code {error_code:?}");
        }

        hlt_loop();

        // println!("{stack_frame:#?}");
    }

    pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
        println!("DOUBLE FAULT:");
        println!("error code {error_code:?}");
        println!("{stack_frame:#?}");

        panic!("double fault");
    }
}
