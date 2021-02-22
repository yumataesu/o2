mod framework;
mod app;

fn main() {
    framework::Runner::new(app::App::default()).run();
}

