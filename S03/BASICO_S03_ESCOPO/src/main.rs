fn exemplo() {
    let s1 = "primeiro string literal";

    {
        let s2 = "segundo string literal";
        println!("{}", s1);
        println!("{}", s2);
    }

    println!("{}", s1);
}

fn main() {
    exemplo();
}
