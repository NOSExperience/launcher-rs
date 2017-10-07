extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationFlags};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Inhibit, WindowPosition};

const TITLE: &str = "NOSE Launcher";

fn main() {
    let app = Application::new(None, ApplicationFlags::empty()).unwrap();
    //app.set_menubar();

    let win = ApplicationWindow::new(&app);
    win.set_title(TITLE);
    win.set_show_menubar(true);
    win.set_default_size(800, 500);
    win.set_position(WindowPosition::Center);
    win.show_all();

    /*
    win.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    gtk::main();
    */
}
