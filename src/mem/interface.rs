use std::fmt;

/// Cheat interface.
pub trait Interface {
    fn read_memory(&mut self, address: u64, size: u64) -> Vec<u8>;
    fn write_memory(&mut self, address: u64, size: u64) -> Vec<u8>;

}