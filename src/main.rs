// Note: https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "io.github.Hydriam.Workshopfetcher-rs";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Workshop Fetcher")
        .build();

    // Present window
    window.present();
}