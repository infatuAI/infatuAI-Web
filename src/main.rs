extern crate cef_sys;
extern crate figlet_rs;

use figlet_rs::{FIGfont};

fn main() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("infatuAI Web");
    println!("{}", figure.unwrap());
}