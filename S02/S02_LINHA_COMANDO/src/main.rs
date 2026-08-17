use std::env;

fn main() {
    println!("Total de elementos em env:args é {}", env::args().len());

    println!("Percorre usando iterador:");
    let mut i = 0;

    for x in env::args() {
        println!("Argumento [{}] == {}", i, x);
        i += 1;
    }

    println!("Percorre usando iterador com indices:");
    for(i, x) in env::args().enumerate() {
        println!("Argumentos [{}] == {}", i, x);
    }

    println!("Coloca tudo em um vector");
    let argumentos: Vec<String> = env::args().collect();

    println!("Percorre usando o vector:");
    for i in 0..argumentos.len() {
        println!("Com vector fica Argumento [{}] == {}", i, argumentos[i]);
    }
    println!("Total de {} elementos no vector", argumentos.len());
}
