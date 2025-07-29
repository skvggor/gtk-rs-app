use gtk::{Application, glib};
use gtk::{ApplicationWindow, prelude::*};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let sum = 1 + 1;
    let app_name = "Skvggor's App".to_string() + " " + &sum.to_string();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(app_name)
        .build();

    window.present();
}
