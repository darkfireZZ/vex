use {
    crate::num_traits::{One, Scalar, Zero},
    std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub mod aliases {
    use super::Vector;

    pub type Vector2f = Vector<2, f32>;
    pub type Vector3f = Vector<3, f32>;

    pub type Vector2d = Vector<2, f64>;
    pub type Vector3d = Vector<3, f64>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector<const N: usize, T> {
    components: [T; N],
}

impl<const N: usize, T> Vector<N, T>
where
    T: Zero + Copy,
{
    pub fn zero() -> Self {
        Self {
            components: [T::ZERO; N],
        }
    }
}

impl<const N: usize, T> Vector<N, T>
where
    T: One + Copy,
{
    pub fn ones() -> Self {
        Self {
            components: [T::ONE; N],
        }
    }
}

impl<const N: usize, T> Vector<N, T>
where
    T: Scalar + Copy,
{
    pub fn norm_squared(&self) -> T {
        // TODO: This panics if N == 0
        let mut acc = self.components[0] * self.components[0];
        for component in &self.components[1..] {
            acc += *component * *component;
        }
        acc
    }

    pub fn norm(&self) -> T {
        self.norm_squared().sqrt()
    }
}

impl<const N: usize, T> Add for Vector<N, T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize, T> AddAssign for Vector<N, T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for index in 0..self.components.len() {
            self.components[index] = self.components[index] + rhs.components[index];
        }
    }
}

impl<const N: usize, T> Sub for Vector<N, T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize, T> SubAssign for Vector<N, T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for index in 0..self.components.len() {
            self.components[index] = self.components[index] - rhs.components[index];
        }
    }
}

impl<const N: usize, T> Neg for Vector<N, T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(mut self) -> Self {
        for component in &mut self.components {
            *component = -*component;
        }
        self
    }
}

impl<const N: usize, T> Mul<T> for Vector<N, T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(mut self, scalar: T) -> Self {
        self *= scalar;
        self
    }
}

impl<const N: usize, T> MulAssign<T> for Vector<N, T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        for component in &mut self.components {
            *component = *component * scalar;
        }
    }
}

impl<const N: usize, T> Div<T> for Vector<N, T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(mut self, scalar: T) -> Self {
        self /= scalar;
        self
    }
}

impl<const N: usize, T> DivAssign<T> for Vector<N, T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, scalar: T) {
        for component in &mut self.components {
            *component = *component / scalar;
        }
    }
}
