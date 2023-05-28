const G: f64 = 9.81;

fn simulate(
    start_angle: f64,
    start_speed: f64,
    timestep: f64,
    time_to_simulate: f64,
    pendulum_length: f64,
) -> Vec<[f64; 2]> {
    let steps = (time_to_simulate / timestep).floor() as usize;
    let mut res = Vec::with_capacity(steps);
    let mut theta = start_angle;
    let mut angular_velocity = start_speed;
    for step in 0..steps {
        let angular_acceleration = -G / pendulum_length * theta.sin();
        angular_velocity += angular_acceleration * timestep;
        theta += angular_velocity * timestep;
        res.push([(step as f64) * timestep, theta]);
    }
    return res;
}

fn main() {
    let data = simulate(
        3.124, /* 3.124 radians ≈ 179° */
        0.0, 0.001, 10.0, 1.0,
    );
    println!(
        "{}",
        data.iter()
            .map(|[time, angle]| format!("{time};{angle}\n"))
            .collect::<String>()
    );
}
