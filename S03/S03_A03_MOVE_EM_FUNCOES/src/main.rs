fn main() {
    let s = String::from("hello");

    //recebe_ownership(s);

    recebe_ownership(s.clone());
    println!("{}", s);

    let x = 5;
    recebe_copia(x);

    println!("{}", x);
}

fn recebe_ownership(um_string: String) {
    println!("{}", um_string);
}

fn recebe_copia(um_inteiro: i32) {
    println!("{}", um_inteiro);
}
