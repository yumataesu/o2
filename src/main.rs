mod framework;
mod app;

fn main() {
    let mut settings = framework::WindowSettings::new();
    settings.gl_version = (4, 5);
    settings.window_size = (1920, 1080);
    settings.title = String::from("Main");

    framework::Runner::new(app::App::default(), settings).run();
}
