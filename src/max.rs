pub trait Max {
    fn maximum() -> Self;
}

macro_rules! impl_max {
    ($t: ty) => {
        impl Max for $t {
            fn maximum() -> Self {
                <$t>::MAX
            }
        }
    };
}

impl_max!(i16);
impl_max!(u16);
impl_max!(i32);
impl_max!(u32);
impl_max!(i64);
impl_max!(u64);
impl_max!(isize);
impl_max!(usize);
