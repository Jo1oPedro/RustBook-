fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Semântica move
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s3 = s2.clone();
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
}
