#![feature(test)]
// #![feature(stdarch_x86_avx512)]

mod compact;
pub use crate::compact::{compact, par_compact};

mod contains;
pub use crate::contains::contains;

mod ops;
pub use crate::ops::{swap, ObliviousOps};

mod shuffle;
pub use crate::shuffle::shuffle;

mod sort;
pub use crate::sort::{par_sort, sort};

mod max;
pub use crate::max::Max;
