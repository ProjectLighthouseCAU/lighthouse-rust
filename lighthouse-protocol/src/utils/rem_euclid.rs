/// A type that supports Euclidean modulo.
pub trait RemEuclid {
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_rem_euclid {
    ($($tys:ty),*) => {
        $(impl RemEuclid for $tys {
            fn rem_euclid(self, rhs: Self) -> Self {
                <$tys>::rem_euclid(self, rhs)
            }
        })*
    };
}

impl_rem_euclid!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);
