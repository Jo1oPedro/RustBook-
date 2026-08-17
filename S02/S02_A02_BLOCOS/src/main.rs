fn main() {
    println!("Hello, world!");

    const X:i32 = 5;
    let y = 6;
    let mut z  = 7;
    z = z + 1;

    println!("No início os valores são: X={X}, y={y}, z={z}");

    {
        const X:i32 = 444;
        let y = 555;
        let mut z = 777;
        z = z + 1;

        println!("Dentro do bloco interno, os valores são: X={X}, y={y}, z={z}");
    }

    println!("Dentro do bloco externo, os valores são: X={X}, y={y}, z={z}");
}
