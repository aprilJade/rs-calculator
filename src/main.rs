use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(480)
            .default_height(800)
            .title("Calculator")
            .build();

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}