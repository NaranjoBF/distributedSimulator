pub mod math;

pub use math::{Scalar, Vec2D};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub balls: Vec<Ball>,
}

impl World {
    pub const GRAVITY: Vec2D = Vec2D {
        x: Scalar::ZERO,
        y: Scalar::from_bits((98i64 << 32) / 10),
    };

    pub fn advance_by_delta_secs(&mut self, secs: Scalar) {
        self.balls.iter_mut().for_each(|ball| {
            ball.velocity += secs * Self::GRAVITY;
            ball.position += secs * ball.velocity;
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ball {
    pub position: Vec2D,
    pub velocity: Vec2D,
    radius: Scalar,
    mass: Scalar,
}

impl Ball {
    pub fn new(p: Vec2D, v: Vec2D, r: Scalar, m: Scalar) -> Result<Self, BallError> {
        if r < Scalar::ZERO {
            return Err(BallError::NegativeRadius);
        }
        if m < Scalar::ZERO {
            return Err(BallError::NegativeMass);
        }

        return Ok(Self {
            position: p,
            velocity: v,
            radius: r,
            mass: m,
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BallError {
    NegativeRadius,
    NegativeMass,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ball_accepts_changes_of_position() {
        // Arrange
        let mut ball = Ball::new(
            Vec2D::new(0, 0),
            Vec2D::new(0, 0),
            Scalar::from(1),
            Scalar::from(1),
        )
        .unwrap();

        // Act
        ball.position.x = 10.into();

        // Assert
        assert_eq!(ball.position.x, 10);
    }

    #[test]
    fn ball_accepts_changes_of_velocity() {
        // Arrange
        let mut ball = Ball::new(
            Vec2D::new(0, 0),
            Vec2D::new(0, 0),
            Scalar::from(1),
            Scalar::from(1),
        )
        .unwrap();

        // Act
        ball.velocity.x = 10.into();

        // Assert
        assert_eq!(ball.velocity.x, 10);
    }

    #[test]
    fn ball_doesnt_accept_negative_radius() {
        // Act
        let result = Ball::new(
            Vec2D::new(0, 0),
            Vec2D::new(0, 0),
            Scalar::from(-10),
            Scalar::from(1),
        );

        // Assert
        assert_eq!(result, Err(BallError::NegativeRadius))
    }

    #[test]
    fn ball_doesnt_accept_negative_mass() {
        // Act
        let result = Ball::new(
            Vec2D::new(0, 0),
            Vec2D::new(0, 0),
            Scalar::from(1),
            Scalar::from(-10),
        );

        // Assert
        assert_eq!(result, Err(BallError::NegativeMass))
    }
}
