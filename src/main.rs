use crate::p_complex::complex::complex;

pub mod p_complex;

fn main() {
    let a = complex::add(2, 2);
    println!("Hello, world! {a}");
}
