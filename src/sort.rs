// pub fn sort<T: std::cmp::PartialOrd>(list: &mut [T]) {
//     bitonic_sort(list, true);
// }

// // Implements bitonic sort.
// fn bitonic_sort<T: std::cmp::PartialOrd>(list: &mut [T], b: bool) {
//     if list.len() > 1 {
//         let middle = list.len() / 2;
//         bitonic_sort(&mut list[0..middle], b);
//         bitonic_sort(&mut list[middle..], !b);
//         bitonic_merge(list, b);
//     }
// }

// fn bitonic_merge<T: std::cmp::PartialOrd>(list: &mut [T], b: bool) {
//     if list.len() > 1 {
//         let m = 1 << list.len().ilog(2);
//         for i in 0..(list.len() - m) {
//             // cmp_swap(&mut list[m], &mut list.split_at_mut(), b)
//         }
//     }
// }

// fn swap<T: std::cmp::PartialOrd>(a: &mut T, b: &mut T, swap: bool) {
//     todo!();
// }

// pub fn compact<T>(list: &mut [T], pred: impl Fn(&T) -> bool) {}

// pub fn cmp<T>(a: T, b: T) -> i8
// where
//     T: std::ops::Sub + std::ops::Shr,
// {
//     use std::mem;

//     let bit_length = 8 * mem::size_of::<T>();
//     -((a - b) >> bit_length - 1) + ((b - a) >> bit_length - 1)
// }

// TODO is this really the best way to do this? it feels ugly. i'd like to be
// able to have these optimized implementations for particular types and then a
// generic implementation if there is not already a better one, by interpreting
// everything as bytes and then doing the arithmetic per byte.
