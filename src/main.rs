extern crate hyper;
extern crate select;
extern crate term;

mod phoronix {
    pub mod article;
    pub mod homepage;
    pub mod cli;
}
mod linesplit;
use std::process;
use phoronix::cli;

fn main() {
    let args = std::env::args().skip(1);
    for argument in args {
        match argument.as_str() {
            "-n" | "--no-color" => cli::print(),
            "-h" | "--help" => print_help(),
            "g" | "--gui" => launch_gui(),
            _ => println!("phoronix-reader: option '{}' not valid", argument)
        }
    }
    cli::print_colored();
}

fn launch_gui() {
    println!("GUI support was removed until further notice.");
    process::exit(0);
}

fn print_help() {
    print!("Prints the latest information from Phoronix.\n");
    print!("    -h, --help     : show this information\n");
    print!("    -g, --gui      : launches a GTK3 GUI instead of outputting to the terminal (deprecated)\n");
    println!("    -n, --no-color : prints to stdout without using colors");
}
