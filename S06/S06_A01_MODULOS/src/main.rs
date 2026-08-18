mod front_of_house;

fn main() {
    println!("Hello, world!");
    println!("Funcao desse modulo: {}", outra_funcao());
    println!("Função do módulo front_of_house: {}", front_of_house::init_front_of_house());
    println!("Função do submódulo hosting: {}", front_of_house::hosting::add_to_waitlist());
}

fn outra_funcao() -> String {
    String::from("outra_funcao")
}
