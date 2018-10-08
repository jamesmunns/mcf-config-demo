// Note: not needed once 1.31 is released
#![feature(min_const_fn)]

pub const fn demo_buffer_size() -> usize {
    16
}

pub const fn operation_mode() -> OperationMode {
    OperationMode::FancyMode
}

pub enum OperationMode {
    DefaultBoringMode,
    FancyMode,
    SpookyMode,
}