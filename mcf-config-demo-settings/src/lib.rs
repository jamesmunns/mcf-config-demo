// Note: not needed once 1.31 is released
#![feature(min_const_fn)]

pub const fn demo_buffer_size() -> usize {
    8
}

pub const fn operation_mode() -> OperationMode {
    OperationMode::DefaultBoringMode
}

pub enum OperationMode {
    DefaultBoringMode,
    FancyMode,
    SpookyMode,
}
