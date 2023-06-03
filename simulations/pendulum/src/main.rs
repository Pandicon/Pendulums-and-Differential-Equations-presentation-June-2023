use eframe::egui;

mod application;
mod structs;

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	dotenv::dotenv().ok();
	let args = std::env::args().collect::<Vec<String>>();
	let vsync_args = args.len() > 1 && args[1].to_lowercase() == *"--vsync-on";
	let native_options = eframe::NativeOptions {
		maximized: true,
		resizable: true,
		vsync: (vsync_args
			|| match std::env::var("VSYNC") {
				Ok(val) => val.to_lowercase() == "on",
				Err(_) => false,
			}),
		..Default::default()
	};

	eframe::run_native(
		"Pendulum simulation",
		native_options,
		Box::new(|cc| Box::new(application::Application::new(cc, VERSION.to_string()))),
	).expect("Failed to start the application");
}

impl eframe::App for application::Application {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state.running {
            self.state.steps_left += self.settings.steps_per_frame;
            for _ in 0..(self.state.steps_left.floor() as usize) {
                self.step();
            }
            self.state.steps_left -= self.state.steps_left.floor();
        }
		while self.settings.limit_saved_points && self.past_displacement_points.len() > self.settings.max_saved_points {
			self.past_displacement_points.remove(0);
		}
		self.render(ctx);
		ctx.request_repaint();
	}
}