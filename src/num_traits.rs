use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

pub trait Scalar:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Sized
{
    fn sqrt(self) -> Self;
}

macro_rules! impl_primitive {
    ( $( $primitive:ty ),* ) => {
        $(
            impl Zero for $primitive {
                const ZERO: Self = 0 as $primitive;
            }

            impl One for $primitive {
                const ONE: Self = 1 as $primitive;
            }
        )*
    }
}

impl_primitive! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

macro_rules! impl_float {
    ( $( $float:ty ),* ) => {
        $(
            impl Scalar for $float {
                fn sqrt(self) -> Self {
                    <$float>::sqrt(self)
                }
            }
        )*
    }
}

impl_float! {
    f32, f64
}
