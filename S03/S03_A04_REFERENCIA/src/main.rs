fn main() {
    let palavra = String::from("abacaxi");

    let len1 = calcula_tamanho_move(palavra.clone());
    println!("tamanho de {} é {}", palavra, len1);

    let len2 = calcula_tamanho_referencia(&palavra);
    println!("tamanho de {} é {}", palavra, len2);
}


fn calcula_tamanho_move(palavra: String) -> usize {
    palavra.len()
}

fn calcula_tamanho_referencia(palavra: &String) -> usize {
    palavra.len()
}