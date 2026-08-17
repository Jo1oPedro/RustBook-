fn main() {
    let a = [11, 22, 33, 44, 55];
    let slice = &a[1..=3];

    for elemento in slice {
        println!("{elemento}");
    }
}
