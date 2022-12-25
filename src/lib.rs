mod vector;
mod complex;

pub use vector::*;
pub use complex::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_complex_arithmetic() {
        let c1 = Complex::new(1, 4);
        let c2 = Complex::new(7, 8);
        
        // Addition
        let mut c3 = c1 + c2;

        assert_eq!(c3, Complex::new(8, 12));

        c3 += c1;

        assert_eq!(c3, Complex::new(9, 16));

        // Subtraction
        let mut c3 = c1 - c2;

        assert_eq!(c3, Complex::new(-6, -4));

        c3 -= c1;

        assert_eq!(c3, Complex::new(-7, -8));

        // Multiplication
        let mut c3 = c1 * c2;

        assert_eq!(c3, Complex::new(-25, 36));

        c3 *= c1;

        assert_eq!(c3, Complex::new(-169, -64));

        // Division
        let c1 = Complex::new(2.0, 4.0);
        let c2 = Complex::new(6.0, 8.0);

        let mut c3 = c1 / c2;

        assert_eq!(c3, Complex::new(0.44, 0.08));

        c3 /= c1;

        assert_eq!(c3, Complex::new(0.06, -0.08));
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(1, 2);
        let d = c.conj();

        assert_eq!(d, Complex::new(1, -2));
    }


}