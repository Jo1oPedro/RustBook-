fn main() {
    let s = String::from("hello");
    change1(&s);

    let mut x = String::from("hello");
    change2(&mut x);
    println!("{}", x);

    let mut y = String::from("hello");
    let ry1 = &y;
    println!("ry1: {}", ry1);
    let ry3  = &mut y;
    println!("ry3: {}", ry3);
}

fn change1(s: &String) {
    //s.push_str(", world");
}

fn change2(s: &mut String) {
    s.push_str(", world");
}