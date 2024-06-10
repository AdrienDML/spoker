
const MAX_VELOCITY: f32 = 14.0;
fn accelerate_bunny_hop(
    wish_dir: Vec3,
    velocity: &mut Velocity,
    acceleration: f32,
    dt: f32,
) -> Vec3 {
    let projected_speed = wish_dir.dot(**velocity);
    let mut added_velocity = acceleration * dt;
    if projected_speed + added_velocity > MAX_VELOCITY {
        added_velocity = MAX_VELOCITY - projected_speed;
    }
    **velocity + added_velocity * wish_dir
}

fn accelerate_strafe(wish_dir: Vec3, velocity: Vec3, dt: f32) -> Vec3 {
    let projected_speed = wish_dir.dot(velocity);
    let add_speed = (RUN_SPEED - projected_speed).clamp(0.0, MAX_ACCEL);
    velocity + add_speed * wish_dir * dt
}

// TODO: Figure out fall detection.
fn air_accelerate(velocity: &mut Velocity, wish_dir: Vec3, wish_speed: f32, dt: f32) {
    let current_speed = wish_dir.dot(**velocity);
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 {
        return;
    }
    let acceleration_speed = (AIR_ACCELERATION * dt * wish_speed).min(add_speed);
    **velocity += acceleration_speed * wish_dir;
}
