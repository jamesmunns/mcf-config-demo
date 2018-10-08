// Note: not needed once 1.31 is released
#![feature(min_const_fn)]

extern crate mcf_config_demo_settings as settings;

use mcf_config_demo_settings::{operation_mode, OperationMode};

const BUF_SZ: usize = settings::demo_buffer_size();
pub static BUFFER: [u8; BUF_SZ] = [0xAC; BUF_SZ];

pub fn greets() {
    use self::OperationMode::*;
    match operation_mode() {
        DefaultBoringMode => println!("Hello, world!"),
        FancyMode => println!("y halo thar!"),
        SpookyMode => println!("2hello4me"),
    }
}