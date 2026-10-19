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

#[test]
fn the_world_updates_all_entities_pos_and_vel_correctly() {
    // Arrange
    let initial_pos = Vec2D::new(5, 10);
    let initial_vel = Vec2D::new(1, 2);

    let balls = (0..3)
        .map(|i| {
            Ball::new(
                initial_pos * i.into(),
                initial_vel * i.into(),
                1.into(),
                2.into(),
            )
            .unwrap()
        })
        .collect();
    let mut world = World { balls };
    let dt = Scalar::from(2);

    // Act
    world.advance_by_delta_secs(dt);

    // Assert
    assert_eq!(world.balls.len(), 3);
    for i in 0..3 {
        let this_ball = &world.balls[i];
        let this_ball_initial_v = Scalar::from_num(i) * initial_vel;
        let this_ball_initial_p = Scalar::from_num(i) * initial_pos;
        let this_ball_expected_v = this_ball_initial_v + dt * World::GRAVITY;

        assert_eq!(this_ball.velocity, this_ball_expected_v);
        assert_eq!(
            this_ball.position,
            this_ball_initial_p + dt * this_ball_expected_v
        )
    }
}
