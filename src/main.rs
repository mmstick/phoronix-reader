extern crate hyper;
extern crate select;
extern crate term;
extern crate getopts;

mod phoronix {
    pub mod article;
    pub mod homepage;
    pub mod cli;
}
mod linesplit;
use phoronix::cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("n", "no-color", "prints without colors");
    opts.optflag("h", "help", "show this information");
    opts.optflag("g", "gui", "display in a GTK3 GUI (deprecated)");
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") { print_help(); return; }
    match matches.opt_present("g") {
        true => println!("GUI support was removed until further notice."),
        false => if matches.opt_present("n") { cli::print(); } else { cli::print_colored(); },
    };
}

fn print_help() {
    println!("Prints the latest information from Phoronix.");
    println!("    -h, --help     : show this information");
    println!("    -g, --gui      : launches a GTK3 GUI instead of outputting to the terminal (deprecated)");
    println!("    -n, --no-color : prints to stdout without using colors");
}
