fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("Usando for");

    let a = [1,2,3,4,5];

    for number in a {
        println!("{}", number);
    }

    println!("Usando range");

    for number in 1..=4 {
        println!("{}", number);
    }

    println!("Usando range reverso");

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("Hello, world!");
}
