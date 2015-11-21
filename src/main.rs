extern crate hyper;
extern crate select;
extern crate term;
mod article;
mod linesplit;
mod phoronix;

fn main() {
    phoronix::print_homepage_colored();
}

