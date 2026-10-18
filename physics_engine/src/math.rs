use fixed::types::I32F32;

pub type Scalar = I32F32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2D {
    pub fn new<T: Into<Scalar>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
