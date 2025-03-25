#![feature(test)]
// #![feature(stdarch_x86_avx512)]

mod compact;
pub use crate::compact::compact;

mod contains;
pub use crate::contains::contains;

mod ops;
pub use crate::ops::{swap, ObliviousOps};

mod sort;
pub use crate::sort::sort;

mod max;
pub use crate::max::Max;
