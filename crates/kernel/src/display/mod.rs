use bootloader_api::{info::FrameBuffer};

use crate::{println, color};

pub mod framebuffer;
pub mod vga;


pub fn init(fb: Option<&'static mut FrameBuffer>) {
    let green = color::ColorName::Green.ansi();
    let clear = color::ColorName::Foreground.ansi();

    // Framebuffer Output
    if let Some(fb) = fb {
        let fb_info = fb.info();

        framebuffer::init(fb.buffer_mut(), fb_info);

        println!("INIT: Framebuffer... [{green}OK{clear}]");
        println!("  Dimensions: w {}, h {}", fb_info.width, fb_info.height);
        println!("  Pixel Format: {:?}", fb_info.pixel_format);
        println!("  Pixel Size: {}", fb_info.bytes_per_pixel);
        println!("  Line Stride: {}", fb_info.stride);
    }
}