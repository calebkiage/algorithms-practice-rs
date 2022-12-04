pub mod math {
    use std::ops::{Rem, Sub};

    trait Number<T>: Eq + Rem<Output=T> + Default + Copy {}

    /// Calculates the GCD of 2 numbers. (Euclidean algorithm)
    ///
    /// # Examples
    /// ```rust
    /// # use mathematical::math::gcd;
    /// # fn main() -> Result<(), &'static str> {
    /// let a = 20;
    /// let b = 30;
    /// let result = gcd(a, b)?;
    /// assert_eq!(result, 10);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Failures
    /// If both a and b are 0, the function fails
    pub fn gcd<'a, T: PartialEq + Rem<Output=T> + Default + Copy + PartialOrd + Sub<Output=T>>(a: T, b: T) -> Result<T, &'a str> {
        if a == Default::default() && b == Default::default() {
            return Err("values cannot be 0");
        }

        let x = gcd_loop(a, b);

        return if a < Default::default() || b < Default::default() {
            let zero: T = Default::default();
            let neg_x = zero - x;
            Ok(max(x, neg_x))
        } else { Ok(x) };
    }

    fn gcd_loop<T: PartialEq + Rem<Output=T> + Default + Copy>(mut x: T, mut y: T) -> T {
        while y != Default::default() {
            let t = y;

            y = x % y;
            x = t;
        }

        x
    }

    fn max<T: PartialOrd>(a: T, b: T) -> T {
        return if a > b {
            a
        } else {
            b
        };
    }
}

#[cfg(test)]
mod tests {
    mod gcd_tests {
        use crate::math::gcd;

        #[test]
        fn test_with_0() {
            let res = gcd(0, 0);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), "values cannot be 0");
        }

        #[test]
        fn test_gcd() {
            let result = gcd(-1386, 3213);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 63);
        }

        #[test]
        fn test_gcd_unsigned() {
            let result = gcd(20u8, 30u8);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 10);
        }

        #[test]
        fn test_gcd_neg() {
            let result = gcd(-20, -30);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 10);
        }

        #[test]
        fn test_gcd_float() {
            let result = gcd(-20f32, 30f32);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 10f32);
        }
    }
}
