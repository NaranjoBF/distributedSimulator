use physics_engine::Ball;
use physics_engine::Scalar;
use physics_engine::Vec2D;
use physics_engine::World;

#[test]
fn a_world_can_be_created_with_initial_conditions() {
    // Arrange
    let ball = Ball::new(
        Vec2D::new(10, 0),
        Vec2D::new(0, 0),
        Scalar::from(5),
        Scalar::from(1),
    )
    .unwrap();

    // Act
    let world = World {
        balls: vec![ball.clone()],
    };

    // Assert
    assert_eq!(world.balls, [ball]);
}
