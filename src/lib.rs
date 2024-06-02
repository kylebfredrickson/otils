#![feature(test)]
// #![feature(stdarch_x86_avx512)]

mod ops;
pub use crate::ops::{swap, ObliviousOps};

mod sort;
pub use crate::sort::sort;

mod compact;
pub use crate::compact::compact;

mod max;
pub use crate::max::Max;
