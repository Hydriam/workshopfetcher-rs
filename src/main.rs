// Note: https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
mod ui;
use gtk::prelude::*;
use gtk::{Application, glib};

use ui::build_ui::build_ui;

const APP_ID: &str = "io.github.Hydriam.Workshopfetcher-rs";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}