//! Shims Module
//!
//! Corresponds to packages/compiler-cli/src/ngtsc/shims

pub mod src;
#[cfg(test)]
mod test;

pub use src::*;
