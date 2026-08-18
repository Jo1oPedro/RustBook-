pub mod hosting;

mod serving;

pub fn init_front_of_house() -> String {
    String::from("init_front_of_house")
}

fn chamadas_caminhos() {
    crate::outra_funcao();
    super::outra_funcao();

    println!("Caminho absoluto: {}", crate::front_of_house::hosting::add_to_waitlist());
    println!("Caminho relativo: {}", hosting::add_to_waitlist());
}