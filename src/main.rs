extern crate fpinscalainrust;

use fpinscalainrust::getting_started;

fn main() {
    println!("Hello, world!");
    println!("{}", getting_started::my_module::format_abs(-42));
}
