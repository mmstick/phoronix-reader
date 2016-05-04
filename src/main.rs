extern crate hyper;
extern crate select;
extern crate term;
#[cfg(feature = "enable_gtk")] extern crate gtk;
#[cfg(feature = "enable_gtk")] extern crate gdk;

mod phoronix {
    pub mod article;
    pub mod homepage;
    pub mod cli;
    #[cfg(feature = "enable_gtk")] pub mod gtk;
}
mod linesplit;
use std::process;
use phoronix::cli;

fn main() {
    let args = std::env::args().skip(1);
    for argument in args {
        match argument.as_str() {
            "-n" | "--no-color" => cli::print(),
            "-h" | "--help"     => print_help(),
            "-g" | "--gui"       => launch_gui(),
            _                   => println!("phoronix-reader: option '{}' not valid", argument)
        }
    }
    cli::print_colored();
}

#[cfg(feature = "enable_gtk")]
fn launch_gui() {
    phoronix::gtk::launch_gtk();
    process::exit(0);
}

#[cfg(not(feature = "enable_gtk"))]
fn launch_gui() {
    println!("phoronix-media: GTK3 GUI support was disabled in this build.");
    process::exit(0);
}

fn print_help() {
    print!("Prints the latest information from Phoronix.\n");
    print!("    -h, --help     : show this information\n");
    print!("    -g, --gui      : launches a GTK3 GUI instead of outputting to the terminal (WIP)\n");
    println!("    -n, --no-color : prints to stdout without using colors");
}
