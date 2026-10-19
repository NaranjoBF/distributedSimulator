use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

use fixed::types::I32F32;

pub type Scalar = I32F32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2D {
    pub fn new<X: Into<Scalar>, Y: Into<Scalar>>(x: X, y: Y) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<Scalar> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2D> for Scalar {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<Scalar> for Vec2D {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition_works() {
        // Arrange
        let test_cases = [
            // (operand 1, operand 2, expected result)
            (Vec2D::new(0, 0), Vec2D::new(1, 2), Vec2D::new(1, 2)),
            (Vec2D::new(-1, -2), Vec2D::new(1, 2), Vec2D::new(0, 0)),
        ];

        // Act and Assert
        for (op1, op2, expected) in test_cases {
            let actual = op1 + op2;
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn vector_subtraction_works() {
        // Arrange
        let test_cases = [
            // (operand 1, operand 2, expected result)
            (Vec2D::new(0, 0), Vec2D::new(1, 2), Vec2D::new(-1, -2)),
            (Vec2D::new(-1, -2), Vec2D::new(1, 2), Vec2D::new(-2, -4)),
        ];

        // Act and Assert
        for (op1, op2, expected) in test_cases {
            let actual = op1 - op2;
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn vector_add_assign_works() {
        // Arrange
        let mut test_vec = Vec2D::new(0, 1);

        // Act
        test_vec += Vec2D::new(1, 2);

        // Assert
        assert_eq!(test_vec, Vec2D::new(1, 3));
    }

    #[test]
    fn vector_multiply_by_scalar_works() {
        // Arrange and Act
        let result = Vec2D::new(1, 2) * Scalar::from(3);

        // Assert
        assert_eq!(result, Vec2D::new(3, 6));
    }

    #[test]
    fn scalar_multiply_by_vector_works() {
        // Arrange and Act
        let result = Scalar::from(3) * Vec2D::new(1, 2);

        // Assert
        assert_eq!(result, Vec2D::new(3, 6));
    }

    #[test]
    fn multiply_by_scalar_and_assign_works() {
        // Arrange
        let mut test_vec = Vec2D::new(5, 6);

        // Act
        test_vec *= Scalar::from(5);

        // Assert
        assert_eq!(test_vec, Vec2D::new(25, 30));
    }
}
