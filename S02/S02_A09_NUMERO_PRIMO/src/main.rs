fn numero_primo_while(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;
    let mut d = 2;
    while d <= limite {
        if num % d == 0 {
            return false;
        }
        d += 1;
    }

    return true;
}

fn main() {
    println!("É primo {:?}", if numero_primo_while(3) { "É" } else { "Não é" });
}
