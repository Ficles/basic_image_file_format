use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label};
use once_cell::sync::Lazy;
use std::env;
static ARGS: Lazy<Vec<String>> = Lazy::new(|| env::args().collect());
fn main() {
    let application = Application::builder()
        .application_id("org.biff.viewer")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_activate(|app| {
        //app.open(&[gtk::gio::File::for_path("test")], "");
        let window = ApplicationWindow::new(app);
        window.set_title("Image Name Here");
        window.set_default_size(1280, 720);

        let container = Box::new(gtk::Orientation::Vertical, 10);

        let label1 = Label::new(None);
        label1.set_label("Image Creator Here");
        let label2 = Label::new(None);
        label2.set_label(ARGS[1].as_str());
        //let button = Button::with_label("Click me!");

        container.add(&label1);
        container.add(&label2);
        //container.add(&button);
        window.add(&container);

        //button.connect_clicked(move |_| {
        //&label.set_label("Hello, World!");
        // });

        window.show_all();
    });
    application.connect_open(|app, _file, _hint| {
        app.activate();
    });

    application.run();
}
