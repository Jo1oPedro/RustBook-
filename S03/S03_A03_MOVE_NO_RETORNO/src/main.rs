fn main() {
    let s1 = devolve_ownership();

    let s2 = String::from("hello");
    let s3 = recebe_e_devolve_ownership(s2);

    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
}

fn devolve_ownership() -> String {
    let algo = String::from("hello");
    algo
}

fn recebe_e_devolve_ownership(um_string: String) -> String {
    println!("{}", um_string);
    um_string
}