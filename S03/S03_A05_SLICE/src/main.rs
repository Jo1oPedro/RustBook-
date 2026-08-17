fn main() {
    let s = String::from("hello world");

    let s1 = &s[0..5];
    let s2 = &s[6..11];
    let s3 = &s[..2];
    let s4 = &s[3..];

    let slit = "Hello, world!";

    println!("s1: {}, s2: {}, s3: {}, s4: {}, slit: {}", s1, s2, s3, s4, slit);
}
