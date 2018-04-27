extern crate gio;
extern crate gtk;

use gtk::{ApplicationWindow, Builder, Dialog, MenuItem, MenuItemExt};
use gio::prelude::*;
use gtk::prelude::*;

//on_AppQuit

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn on_app_quit() {
    println!("QUIT called");
}

pub fn get_application() -> gtk::Application {
    let application = gtk::Application::new("org.novelist", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_main(app);
    });
    application.connect_activate(|_| {});
    return application;
}

pub fn build_main(application: &gtk::Application) {
    let ui_src = include_str!("main.ui");
    let builder = Builder::new_from_string(ui_src);

    let window: ApplicationWindow = builder
        .get_object("mainwindow")
        .expect("Couldn't get Window");
    let about: Dialog = builder.get_object("uber").expect("Couldn't get Window");

    window.set_application(application);
    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let quit_menu: MenuItem = builder
        .get_object("mainwindow-menu-file-quit")
        .expect("Couldn't get Quit option");

    quit_menu.connect_activate(clone!(window => move |_| {
        window.destroy();
    }));

    let about_menu: MenuItem = builder
        .get_object("mainwindow-menu-help-about")
        .expect("Couldn't get Quit option");

    about_menu.connect_activate(clone!(window => move |_| {
        about.run();
        about.hide();
    }));

    window.show_all();
}
