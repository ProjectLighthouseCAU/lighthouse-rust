/// A type whose values have a square root.
pub trait Sqrt {
    /// The square root.
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($($tys:ty),*) => {
        $(impl Sqrt for $tys {
            fn sqrt(self) -> Self {
                <$tys>::sqrt(self)
            }
        })*
    };
}

impl_sqrt!(
    f32, f64
);
