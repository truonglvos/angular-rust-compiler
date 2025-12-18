//! File System Abstraction
//!
//! Corresponds to packages/compiler-cli/src/ngtsc/file_system



pub mod src;
pub mod testing;

#[cfg(test)]
mod test;

pub use src::*;
