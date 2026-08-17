#[derive(Debug)]
enum Mensagem {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Mensagem {
    fn call(&self) {
        println!("Mensagem chamada é: {:?}", &self)
    }
}

fn main() {
    let mensagem = Mensagem::Quit;
    mensagem.call();
}
