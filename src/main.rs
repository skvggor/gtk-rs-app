use gtk::{Application, Button, glib};
use gtk::{ApplicationWindow, prelude::*};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let app_name = "Skvggor's App";

    let button = Button::builder()
        .label("Press me!")
        .margin_top(100)
        .margin_end(100)
        .margin_bottom(100)
        .margin_start(100)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Clicked!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title(app_name)
        .child(&button)
        .build();

    window.present();
}
