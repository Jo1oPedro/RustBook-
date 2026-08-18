pub mod serving;
pub mod cooking;
pub mod hosting;

use rand::Rng;

use std::collections::HashMap;

use std::{cmp::Ordering, io, self};

fn outra_funcao() -> String {
    String::from("outra_funcao")
}

fn main() {
    println!("Funcao desse módulo: {}", outra_funcao());
    println!("Cooking::clean_up: {}", cooking::clean_up());
    println!("Cooking::clean_up: {}", hosting::clean_up());

    println!("Número aleatório: {}", rand::thread_rng().gen_range(1..=100));

    println!("Hello, world!");
}
