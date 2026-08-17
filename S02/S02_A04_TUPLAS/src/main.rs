fn main() {
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    let tup2 = (500, 6.4, true);

    println!("Minha tupla tem: {:?}", tup2);

    let (x1, y1, z1) = tup2;
    println!("Minha tupla tem {x1}, {y1}, {z1}");

    println!("Minha tupla tem: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);

    println!("Tupla vazia: {:?}", ());
}
