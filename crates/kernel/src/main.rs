#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate tracing;

use core::panic::PanicInfo;

use bootloader_api::{entry_point, config::Mapping, BootloaderConfig, BootInfo};

#[macro_use] extern crate gbl;

use kernel::{font::{FONTS, validate_fonts}, color::ColorName, task::{Task, Executor, spawn_task}};
use x86_64::instructions::port::Port;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{info}");

    kernel::hlt_loop()
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::FixedAddress(0x0000_F000_0000_0000));
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);

    let mut executor = Executor::new();

    debug!("Debug Display");
    info!("Info Display");
    warn!("Warn Display");
    error!("Error Display");
    trace!("Trace display");

    for (i, glyph) in FONTS.iter().enumerate() {
        if (i + 1) % 10 == 0 {
            println!();
        }

        let color = if i % 2 == 0 {
            ColorName::Blue.ansi()
        } else {
            ColorName::Cyan.ansi()
        };

        print!("{color}{}  ", glyph.charlie);
    }

    println!("{}", ColorName::Foreground.ansi());

    validate_fonts();

    for i in 0..13 {
        println!("ASDF FSAF {}", 10usize.pow(i));
    }

    executor.spawn(Task::new(kernel::task::keyboard::handle_key_presses()));
    executor.spawn(Task::new(kernel::task::output::handle_output()));

    executor.run();
}