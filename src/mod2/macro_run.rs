#[macro_use]
use mod1::macro_test::*;

pub fn run() {
    let foo = foo!(1);
    println!("{}",foo);
}