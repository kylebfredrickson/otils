use crate::ops;
use std::thread;

fn split<'list, 'pad, T>(
    list: &'list mut [T],
    pad: &'pad mut [T],
) -> (&'list mut [T], &'list mut [T], &'pad mut [T], &'pad mut [T]) {
    let list_len = list.len();
    let pad_len = pad.len();

    if list_len >= pad_len {
        let (l, r) = list.split_at_mut((list_len + pad_len) / 2);
        let (l_pad, r_pad) = pad.split_at_mut(0);
        (l, r, l_pad, r_pad)
    } else {
        let (l, r) = list.split_at_mut(list.len());
        let (l_pad, r_pad) = pad.split_at_mut((list_len + pad_len) / 2 - list_len);
        (l, r, l_pad, r_pad)
    }
}

pub fn parallel_padded_bitonic_sort<T: PartialOrd + Send>(
    data: &mut [T],
    pad: &mut [T],
    cond: bool,
    threads: usize,
) {
    if threads <= 1 {
        padded_bitonic_sort(data, pad, cond);
        return;
    }

    if data.len() + pad.len() <= 1 {
        return;
    }

    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    let (l_data, r_data, l_pad, r_pad) = split(data, pad);

    thread::scope(|s| {
        s.spawn(|| parallel_padded_bitonic_sort(l_data, l_pad, cond, l_threads));
        parallel_padded_bitonic_sort(r_data, r_pad, !cond, r_threads);
    });
    parallel_padded_bitonic_merge(l_data, r_data, l_pad, r_pad, cond, threads);
}

fn parallel_padded_bitonic_merge<T: PartialOrd + Send>(
    l_data: &mut [T],
    r_data: &mut [T],
    l_pad: &mut [T],
    r_pad: &mut [T],
    cond: bool,
    threads: usize,
) {
    if threads <= 1 {
        padded_bitonic_merge(l_data, r_data, l_pad, r_pad, cond);
        return;
    }

    if l_data.len() + l_pad.len() < 1 || r_data.len() + r_pad.len() < 1 {
        return;
    }

    padded_bitonic_pass(l_data, r_data, l_pad, r_pad, cond);

    let l_threads = threads / 2;
    let r_threads = threads - l_threads;
    thread::scope(|s| {
        s.spawn(|| {
            let (ll_data, lr_data, ll_pad, lr_pad) = split(l_data, l_pad);
            parallel_padded_bitonic_merge(ll_data, lr_data, ll_pad, lr_pad, cond, l_threads)
        });
        let (rl_data, rr_data, rl_pad, rr_pad) = split(r_data, r_pad);
        parallel_padded_bitonic_merge(rl_data, rr_data, rl_pad, rr_pad, cond, r_threads);
    });
}

pub fn padded_bitonic_sort<T: PartialOrd>(list: &mut [T], pad: &mut [T], cond: bool) {
    if list.len() + pad.len() <= 1 {
        return;
    }

    let (l_half, r_half, l_pad, r_pad) = split(list, pad);
    padded_bitonic_sort(l_half, l_pad, cond);
    padded_bitonic_sort(r_half, r_pad, !cond);
    padded_bitonic_merge(l_half, r_half, l_pad, r_pad, cond);
}

fn padded_bitonic_merge<T: PartialOrd>(
    l_half: &mut [T],
    r_half: &mut [T],
    l_pad: &mut [T],
    r_pad: &mut [T],
    cond: bool,
) {
    if l_half.len() + l_pad.len() < 1 || r_half.len() + r_pad.len() < 1 {
        return;
    }

    padded_bitonic_pass(l_half, r_half, l_pad, r_pad, cond);

    let (ll_quarter, lr_quarter, ll_pad, lr_pad) = split(l_half, l_pad);
    let (rl_quarter, rr_quarter, rl_pad, rr_pad) = split(r_half, r_pad);
    padded_bitonic_merge(ll_quarter, lr_quarter, ll_pad, lr_pad, cond);
    padded_bitonic_merge(rl_quarter, rr_quarter, rl_pad, rr_pad, cond);
}

// This makes it slower for some reason.
// fn parallel_padded_bitonic_pass<T: PartialOrd + Send>(
//     l_half: &mut [T],
//     r_half: &mut [T],
//     cond: bool,
//     threads: usize,
// ) {
//     // need to check that the chunks are big enough also
//     if threads <= 1 {
//         padded_bitonic_pass(l_half, r_half, cond);
//         return;
//     }

//     let l_threads = threads / 2;
//     let r_threads = threads - l_threads;
//     let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
//     let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
//     thread::scope(|s| {
//         s.spawn(|| parallel_padded_bitonic_pass(ll_quarter, rl_quarter, cond, l_threads));
//         parallel_padded_bitonic_pass(lr_quarter, rr_quarter, cond, r_threads);
//     });
// }

#[inline]
fn padded_bitonic_pass<T: PartialOrd>(
    l_half: &mut [T],
    r_half: &mut [T],
    l_pad: &mut [T],
    r_pad: &mut [T],
    cond: bool,
) {
    for i in 0..(l_half.len() + l_pad.len()) {
        let l = if i < l_half.len() {
            &mut l_half[i]
        } else {
            &mut l_pad[i - l_half.len()]
        };

        let r = if i < r_half.len() {
            &mut r_half[i]
        } else {
            &mut r_pad[i - r_half.len()]
        };

        ops::swap((l < r) ^ cond, l, r)
    }
}
