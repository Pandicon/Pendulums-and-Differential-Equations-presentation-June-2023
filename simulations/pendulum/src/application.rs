use crate::structs;
use eframe::egui;

const GRAPH_START_POINT: egui::Vec2 = egui::Vec2::new(0.0, 0.0);
const PENDULUM_GRAPH_GAP: f32 = 1.0;
const G: f64 = 9.81;

pub struct Application {
	pub past_displacement_points: Vec<structs::DisplacementPoint>,
	pub pendulum: structs::Pendulum,
	pub settings: structs::Settings,
	pub state: structs::State,
	pub version: String,
}

impl Application {
	pub fn new(cc: &eframe::CreationContext<'_>, version: String) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		Self {
			past_displacement_points: Vec::new(),
			pendulum: structs::Pendulum::default(),
			settings: structs::Settings::default(),
			state: structs::State::default(),
			version,
		}
	}

	pub fn step(&mut self) {
		self.state.positions_since_save += 1;
		self.state.simulation_time += self.settings.timestep;
		self.pendulum.angular_acceleration = -G / self.pendulum.length * self.pendulum.angle.sin();
		self.pendulum.angular_velocity += self.pendulum.angular_acceleration * self.settings.timestep;
		self.pendulum.angle += self.pendulum.angular_velocity * self.settings.timestep;
		let displacement = (self.pendulum.angle + std::f64::consts::PI).cos() * self.pendulum.length;
		if self.state.positions_since_save > self.settings.save_skip {
			self.past_displacement_points.push(structs::DisplacementPoint::new(displacement, self.state.simulation_time));
			self.state.positions_since_save = 0;
		}
	}

	pub fn render(&mut self, ctx: &egui::Context) {
		self.render_info_window(ctx);
		self.render_settings_window(ctx);
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				let run_pause_button = ui.button(if self.state.running { "Pause the simulation" } else { "Run the simulation" });
				if run_pause_button.clicked() {
					self.state.running = !self.state.running;
				}
				ui.add_space(10.0);
				
				let reset_button = ui.button("Reset the simulation");
				if reset_button.clicked() {
					self.past_displacement_points = Vec::new();
					self.state.simulation_time = 0.0;
				}
				ui.add_space(10.0);
				
				let show_pendulum_info_button = ui.button(if self.state.pendulum_info_active { "Hide pendulum settings" } else { "Show pendulum settings" });
				if show_pendulum_info_button.clicked() {
					self.state.pendulum_info_active = !self.state.pendulum_info_active;
				}
				ui.add_space(10.0);
				
				let show_settings_button = ui.button(if self.state.settings_active { "Hide settings" } else { "Show settings" });
				if show_settings_button.clicked() {
					self.state.settings_active = !self.state.settings_active;
				}
				ui.add_space(10.0);
			});

			// Plot
			let pendulum_angle_adjusted = -self.pendulum.angle + std::f64::consts::PI; // To have 0 degrees at the bottom and 90 degrees on the right
			let plot = egui::plot::Plot::new("Displacement").data_aspect(1.0);

			let pendulum_centre_point = GRAPH_START_POINT - eframe::epaint::Vec2::new(self.pendulum.length as f32 + PENDULUM_GRAPH_GAP, 0.0);
			let (pendulum_x, pendulum_y) = pendulum_angle_adjusted.sin_cos();
			let pendulum_position = egui::Vec2::new((pendulum_x * self.pendulum.length) as f32, (pendulum_y * self.pendulum.length) as f32) + pendulum_centre_point;
			
			// Pendulum end circle
			let n = 512;
			let penculum_circle_points: egui::plot::PlotPoints = (0..=n)
				.map(|i| {
					let t = eframe::emath::remap(i as f64, 0.0..=(n as f64), 0.0..=std::f64::consts::TAU);
					let r = self.pendulum.radius as f64;
					let (x, y) = t.sin_cos();
					[x * r + pendulum_position.x as f64, y * r + pendulum_position.y as f64]
				})
				.collect();
			let pendulum_circle_line = egui::plot::Line::new(penculum_circle_points).color(eframe::epaint::Color32::RED).highlight(true);
			let pendulum_point = egui::plot::Points::new(vec![[pendulum_position.x as f64, pendulum_position.y as f64]]).color(eframe::epaint::Color32::RED).highlight(true);
			
			// Pendulum connection line
			let pendulum_connection_line = egui::plot::Line::new(vec![
				[pendulum_centre_point.x as f64, pendulum_centre_point.y as f64], [pendulum_position.x as f64, pendulum_position.y as f64]
				]).color(eframe::epaint::Color32::LIGHT_RED).highlight(true);

			// Pendulum circle
			let n = 512;
			let circle_points: egui::plot::PlotPoints = (0..=n)
				.map(|i| {
					let t = eframe::emath::remap(i as f64, 0.0..=(n as f64), 0.0..=std::f64::consts::TAU);
					let r = self.pendulum.length as f64;
					let (x, y) = t.sin_cos();
					[x * r + pendulum_centre_point.x as f64, y * r + pendulum_centre_point.y as f64]
				})
				.collect();
			let circle_line = egui::plot::Line::new(circle_points).color(eframe::epaint::Color32::GRAY).highlight(true);

			// Displacement line
			let displacement_points: egui::plot::PlotPoints = self.past_displacement_points.iter().map(|point| {
				// Using past_displacement_points[0] is safe here since the map won't run on an empty vector, meaning that the length must be at least 1
				[self.past_displacement_points[self.past_displacement_points.len() - 1].time - point.time + GRAPH_START_POINT.x as f64, point.displacement + GRAPH_START_POINT.y as f64]
			}).collect();
			let displacement_line = egui::plot::Line::new(displacement_points).color(eframe::epaint::Color32::YELLOW).highlight(true);

			// Line connecting the pendulum to the displacement line
			let pendulum_displacement_line_connection_line = egui::plot::Line::new(vec![
				[pendulum_position.x as f64, pendulum_position.y as f64], [GRAPH_START_POINT.x as f64, pendulum_position.y as f64]
				]).color(eframe::epaint::Color32::LIGHT_YELLOW).highlight(true);

			// Render the plot
			plot.show(ui, |plot_ui| {
				plot_ui.line(circle_line);

				plot_ui.line(pendulum_displacement_line_connection_line);
				plot_ui.line(displacement_line);

				plot_ui.line(pendulum_connection_line);
				plot_ui.line(pendulum_circle_line);
				plot_ui.points(pendulum_point);
			});
		});
	}

	pub fn render_info_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Pendulum settings").open(&mut self.state.pendulum_info_active).show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("The angular displacement of the pendulum");
				ui.add_enabled_ui(!self.state.running, |ui| {
					let mut angle_deg = self.pendulum.angle * 180.0 / std::f64::consts::PI;
					ui.add(egui::DragValue::new(&mut angle_deg)
							.speed(0.1)
							// .clamp_range(-180.0..=180.0)
							.suffix("°")
							.max_decimals(20));
					self.pendulum.angle = angle_deg * std::f64::consts::PI / 180.0;
				});
			});

			ui.horizontal(|ui| {
				ui.label("The angular velocity of the pendulum");
				ui.add_enabled_ui(!self.state.running, |ui| {
					let mut velocity_deg = self.pendulum.angular_velocity * 180.0 / std::f64::consts::PI;
					ui.add(egui::DragValue::new(&mut velocity_deg)
							.speed(0.1)
							.suffix("°/s")
							.max_decimals(20));
					self.pendulum.angular_velocity = velocity_deg * std::f64::consts::PI / 180.0;
				});
			});

			ui.horizontal(|ui| {
				ui.label("The angular acceleration of the pendulum");
				ui.add_enabled_ui(!self.state.running, |ui| {
					let mut angular_acceleration = self.pendulum.angular_acceleration * 180.0 / std::f64::consts::PI;
					ui.add(egui::DragValue::new(&mut angular_acceleration)
							.speed(0.0)
							.suffix("°/s²")
							.max_decimals(5));
				});
			});

			ui.horizontal(|ui| {
				ui.label("The radius of the pendulum circle");
				ui.add(egui::DragValue::new(&mut self.pendulum.radius)
						.speed(0.01)
						.clamp_range(0.0..=f32::INFINITY)
						.max_decimals(5));
			});

			ui.horizontal(|ui| {
				ui.label("The length of the pendulum");
				ui.add_enabled_ui(!self.state.running, |ui| {
					ui.add(egui::DragValue::new(&mut self.pendulum.length)
						.speed(0.1)
						.clamp_range(0.0..=f64::INFINITY)
						.max_decimals(5));
				});
				
			});
		});
	}

	pub fn render_settings_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Settings").open(&mut self.state.settings_active).show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("The simulation timestep");
				ui.add(egui::DragValue::new(&mut self.settings.timestep)
						.speed(0.1)
						.suffix(" s/step")
						.max_decimals(20));
			});

			ui.horizontal(|ui| {
				ui.label("Simulation steps per frame");
				ui.add(egui::DragValue::new(&mut self.settings.steps_per_frame)
						.speed(0.1)
						.suffix(" steps/frame")
						.max_decimals(20));
			});

			ui.horizontal(|ui| {
				ui.label("Positions to skip before saving");
				ui.add(egui::DragValue::new(&mut self.settings.save_skip)
						.speed(0.1)
						.max_decimals(20));
			});

			ui.horizontal(|ui| {
				ui.label("Limit saved positions");
				ui.add(egui::Checkbox::new(&mut self.settings.limit_saved_points, ""));
			});

			ui.horizontal(|ui| {
				ui.label("Maximum positions to save");
				ui.add(egui::DragValue::new(&mut self.settings.max_saved_points)
						.speed(0.1)
						.max_decimals(20));
			});
		});
	}
}