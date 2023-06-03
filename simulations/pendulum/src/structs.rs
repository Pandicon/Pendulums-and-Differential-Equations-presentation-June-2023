#[derive(Debug)]
pub struct DisplacementPoint {
	pub displacement: f64,
	pub time: f64,
}

impl DisplacementPoint {
	pub fn new(displacement: f64, time: f64) -> Self {
		Self { displacement, time }
	}
}

pub struct Settings {
	pub timestep: f64,
	pub steps_per_frame: f32,
	pub save_skip: usize,
	pub limit_saved_points: bool,
	pub max_saved_points: usize
}

impl Default for Settings {
	fn default() -> Self {
		Self { timestep: 0.01, steps_per_frame: 1.0, save_skip: 0, limit_saved_points: false, max_saved_points: 0 }
	}
}

pub struct Pendulum {
	pub angle: f64,
	pub angular_velocity: f64,
	pub angular_acceleration: f64,
	pub length: f64,
	pub radius: f32,
}

impl Default for Pendulum {
	fn default() -> Self {
		Self { angle: 0.0, angular_velocity: 0.0, angular_acceleration: 0.0, length: 1.0, radius: 0.05 }
	}
}

pub struct State {
	pub running: bool,
	pub steps_left: f32,
	pub simulation_time: f64,
	pub positions_since_save: usize,
	pub pendulum_info_active: bool,
	pub settings_active: bool,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: false,
			steps_left: 0.0,
			simulation_time: 0.0,
			positions_since_save: 0,

			pendulum_info_active: false,
			settings_active: false
		}
	}
}