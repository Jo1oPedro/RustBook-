fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("O valor do x é: {x}");
    let x = x + 1;
    println!("O valor de x é: {x}");

    {
        let x = x * 2;
        println!("O valor de x no bloco interno é: {x}");
    }

    println!("O valor de x depois do bloco interno é: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("O valor de spaces é: {spaces}");

    let mut spaces2 = " ";
    println!("O valor de spaces2 é: {spaces2}");
    spaces2 = "qwerty";
    println!("O valor de spaces2 é: {spaces2}");
}
