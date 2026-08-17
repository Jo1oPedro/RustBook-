fn outra_funcao() {
    println!("outrafuncao");
}

fn outra_funcao_com_parametro(x: i32) {
    println!("outrafuncao com parametro {:?}", x);
}

fn soma(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("Hello, world!");
    outra_funcao();
    outra_funcao_com_parametro(5);
    println!("retorno: {:?}", soma(1, 2));
}
