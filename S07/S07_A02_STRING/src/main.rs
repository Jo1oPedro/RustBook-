fn main() {
    let s = String::new();
    println!("P1: s >>>{s}");

    let s = String::from("initial contents");
    println!("P2: s >>>{s}");

    let data = "conteudo inicial";
    let s = data.to_string();
    println!("P3: s >>>{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("P4: s >>>{s}");


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("P5: s >>>{s1}");
    println!("P6: s >>>{s2}");

    s1.push('l');
    println!("P7: s >>>{s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 lost ownership
    println!("P8: {}", s3);

    let mut s1 = String::from("Hello, ");
    s1.push_str(&s2);
    println!("P9: {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}"); // não tira a ownership de s1
    println!("P10: {}", s4);

    // não é possível indexar strings
    let s5 = String::from("hello");
    //let h = s5[0];

    let hello = "cascata";
    let s6 = &hello[0..4];
    println!("P11: {}", s6);
}
