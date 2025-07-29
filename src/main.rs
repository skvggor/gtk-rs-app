use gtk::{Application, ApplicationWindow, Button, glib, prelude::*};

use std::cell::Cell;
use std::rc::Rc;

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

    let counter = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();

    button.connect_clicked(move |button| {
        let new_value = counter_clone.get() + 1;
        counter_clone.set(new_value);

        button.set_label(&new_value.to_string());
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title(app_name)
        .child(&button)
        .build();

    window.present();
}
