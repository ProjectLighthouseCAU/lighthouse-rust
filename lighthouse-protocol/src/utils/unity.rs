/// A type that has positive and negative unit values.
pub trait Unity {
    /// The unit value.
    const ONE: Self;
    /// The negative unit value.
    const NEG_ONE: Self;
}

macro_rules! impl_int_unity {
    ($($tys:ty),*) => {
        $(impl Unity for $tys {
            const ONE: Self = 1;
            const NEG_ONE: Self = -1;
        })*
    };
}

macro_rules! impl_float_unity {
    ($($tys:ty),*) => {
        $(impl Unity for $tys {
            const ONE: Self = 1.0;
            const NEG_ONE: Self = -1.0;
        })*
    };
}

impl_int_unity!(
    i8, i16, i32, i64, i128, isize
);

impl_float_unity!(
    f32, f64
);
