pub mod prime;

use num::Integer;

/// extgcd(a, b) -> (gcd(a, b), x, y)
/// satisfies ax + by = gcd(a, b)
pub fn extgcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Copy + Integer,
{
    if b != T::zero() {
        let (g, y, x) = extgcd(b, a % b);
        (g, x, y - a / b * x)
    } else {
        (a, T::one(), T::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extgcd_test() {
        assert_eq!(extgcd(111, 30), (3, 3, -11));
        assert_eq!(extgcd(1071, 1029), (21, -24, 25));
        assert_eq!(extgcd(1, 1), (1, 0, 1));
    }
}
