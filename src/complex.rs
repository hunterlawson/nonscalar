use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Complex number in the form of a + bi.
/// Where a is real and b is imaginary
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
pub struct Complex<S> {
    pub re: S,
    pub im: S,
}

impl<S> Complex<S>
where
    S: Copy + Neg<Output = S>,
{
    #[inline]
    pub fn new(re: S, im: S) -> Complex<S> {
        Complex { re, im }
    }

    #[inline]
    pub fn from_tuple(tuple: (S, S)) -> Complex<S> {
        Complex {
            re: tuple.0,
            im: tuple.1,
        }
    }

    /// Returns a new complex number that represents the conjugate of the current number
    #[inline]
    pub fn conj(&self) -> Complex<S> {
        Complex {
            re: self.re,
            im: self.im.neg(),
        }
    }
}

impl<S> Add for Complex<S>
where
    S: Add<Output = S>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<S> AddAssign for Complex<S>
where
    S: Add<Output = S> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<S> Sub for Complex<S>
where
    S: Sub<Output = S>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<S> SubAssign for Complex<S>
where
    S: Sub<Output = S> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<S> Mul for Complex<S>
where
    S: Mul<Output = S> + Add<Output = S> + Sub<Output = S> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<S> MulAssign for Complex<S>
where
    S: Mul<Output = S> + Add<Output = S> + Sub<Output = S> + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<S> Div for Complex<S>
where
    S: Div<Output = S> + Add<Output = S> + Mul<Output = S> + Sub<Output = S> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Complex {
            re: ((self.re * rhs.re) + (self.im * rhs.im)) / (rhs.re * rhs.re + rhs.im * rhs.im),
            im: ((self.im * rhs.re) - (self.re * rhs.im)) / (rhs.re * rhs.re + rhs.im * rhs.im),
        }
    }
}

impl<S> DivAssign for Complex<S>
where
    S: Div<Output = S> + Add<Output = S> + Mul<Output = S> + Sub<Output = S> + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = Complex {
            re: ((self.re * rhs.re) + (self.im * rhs.im)) / (rhs.re * rhs.re + rhs.im * rhs.im),
            im: ((self.im * rhs.re) - (self.re * rhs.im)) / (rhs.re * rhs.re + rhs.im * rhs.im),
        }
    }
}
