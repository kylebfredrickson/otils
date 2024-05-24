use std::arch::asm;

// pub fn swap_i8(cond: bool, a: &mut i8, b: &mut i8) {
//     let cond = cond as u8;
//     let a = a as *mut i8 as *mut i16;
//     let b = b as *mut i8 as *mut i16;
//     unsafe {
//         asm!(
//             "test {cond}, {cond}",
//             "cmovnz {a:x}, {b:x}",
//             "cmovnz {b:x}, {tmp:x}",
//             cond = in(reg_byte) cond,
//             a = inout(reg) *a,
//             b = inout(reg) *b,
//             tmp = in(reg) *a,
//         );
//     }
// }

// pub fn swap_u8(cond: bool, a: &mut u8, b: &mut u8) {
//     let cond = cond as u8;
//     unsafe {
//         asm!(
//             "movzx {a}, {an}",
//             "movzx {b}, {bn}",
//             "mov {tmp}, {tmpn}",
//             "test {cond}, {cond}",
//             "cmovnz {an}, {bn}",
//             "cmovnz {bn}, {tmpn}",
//             "mov {an}, {a}",
//             "mov {bn}, {b}",
//             cond = in(reg_byte) cond,
//             a = inout(reg_byte) *a,
//             b = inout(reg_byte) *b,
//             tmp = in(reg_byte) *a,
//             an = out(reg) _,
//             bn = out(reg) _,
//             tmpn = out(reg) _,
//         );
//     }
// }

pub fn swap_i32(cond: bool, a: &mut i32, b: &mut i32) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:e}, {b:e}",
            "cmovnz {b:e}, {tmp:e}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}

pub fn swap_u32(cond: bool, a: &mut u32, b: &mut u32) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:e}, {b:e}",
            "cmovnz {b:e}, {tmp:e}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}

pub fn swap_i64(cond: bool, a: &mut i64, b: &mut i64) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:r}, {b:r}",
            "cmovnz {b:r}, {tmp:r}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}

pub fn swap_u64(cond: bool, a: &mut u64, b: &mut u64) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:r}, {b:r}",
            "cmovnz {b:r}, {tmp:r}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}

pub fn swap_isize(cond: bool, a: &mut isize, b: &mut isize) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:r}, {b:r}",
            "cmovnz {b:r}, {tmp:r}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}

pub fn swap_usize(cond: bool, a: &mut usize, b: &mut usize) {
    let cond = cond as u8;
    unsafe {
        asm!(
            "test {cond}, {cond}",
            "cmovnz {a:r}, {b:r}",
            "cmovnz {b:r}, {tmp:r}",
            cond = in(reg_byte) cond,
            tmp = in(reg) *a,
            a = inout(reg) *a,
            b = inout(reg) *b,
        );
    }
}
