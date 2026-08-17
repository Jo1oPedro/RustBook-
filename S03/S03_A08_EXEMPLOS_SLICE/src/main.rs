use std::io;

fn tamanho_palavra_v1() {
    println!("Tamanho palavra _v1()");

    let mut lista_palavras = Vec::new();

    loop {
        println!("[V1] Digite uma palavra ou somente enter para terminar");
        let mut linha = String::new();
        io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
        linha = linha.trim().to_string();

        if linha.len() == 0 {
            break;
        } else {
            println!("Lido: {}", linha);
            lista_palavras.push(linha);
        }
    }

    for p in lista_palavras {
        println!("{}", p);
    }
}

fn tamanho_palavra_v2() {
    let mut lista_palavras: Vec<String> = Vec::new();

    loop {
        println!("[V2] Digite várias palavras ou somente enter para terminar");
        let mut linha = String::new();
        io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
        linha = linha.trim().to_string();
        if(linha.len() == 0) {
            break;
        } else {
            let palavras = linha.split_whitespace();
            println!("Palavras: {:?}", linha);
            for p in palavras {
                lista_palavras.push(p.trim().to_string());
            }
        }
    }

    for p in lista_palavras {
        println!("{}", p);
    }
}

fn main() {
    tamanho_palavra_v2();
}
