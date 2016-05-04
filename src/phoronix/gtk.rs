#[cfg(feature = "enable_gtk")] use gtk;
#[cfg(feature = "enable_gtk")] use gtk::prelude::*;
#[cfg(feature = "enable_gtk")] use gtk::{Window,WindowType};

#[cfg(feature = "enable_gtk")]
pub fn launch_gtk() {
    gtk::init().unwrap_or_else(|_| panic!("phoronix-reader: failed to initialize GTK."));
    let window = Window::new(WindowType::Toplevel);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

#[cfg(not(feature = "enable_gtk"))]
pub fn launch_gtk() {
    println!("phoronix-media: GTK3 GUI support was disabled in this build.");
    ::std::process::exit(0);
}
