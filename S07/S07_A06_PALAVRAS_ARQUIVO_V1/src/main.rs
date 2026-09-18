use std::process;
use std::env;

use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;

struct Teste {
    #[allow(dead_code)]
    caixa_alta: String,
    conta: u32
}

impl Teste {
    fn new(chave: &String, conta: u32) -> Teste {
        Teste {
            caixa_alta: chave.clone().to_uppercase(),
            conta,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Uso: s07_a06_lendo_arquivo_txt <nome do arquivo>");
        process::exit(1);
    }

    let mut reader: BufReader<File>;
    println!("Abrindo {} ...", args[1]);
    if let Ok(file) = File::open(&args[1]) {
        reader = BufReader::new(file);
    } else {
        println!("Arquivo não encontrado: {}", args[1]);
        process::exit(1);
    }

    let mut contagem: HashMap<String, Teste> = HashMap::new();

    loop {
        let mut linha_lida = String::new();
        let n_bytes = reader.read_line(&mut linha_lida).expect("Não conseguiu ler o arquivo");

        if n_bytes == 0 {
            //arquivo terminou
            break;
        } else {
            print!("{}", linha_lida);

            let palavras = linha_lida.split_whitespace();
            for p in palavras {
                let mut palavra = String::from(p);

                let ultimo_char = palavra.chars().last().unwrap();

                match ultimo_char {
                    '.' | ',' | ';' | ':' | '?' | '!' => palavra.pop().unwrap(),
                    outras => outras
                };

                println!("{}", palavra);
                if palavra.is_empty() {
                    continue;
                }

                match contagem.get_mut(&palavra) {
                    Some(teste) => teste.conta += 1,
                    None => {
                        contagem.insert(
                            palavra.clone(),
                            Teste::new(&linha_lida, 1)
                        );
                    }
                }
            }
        }
    }

    let mut todos = Vec::new();
    for (key, teste) in contagem.iter() {
        todos.push((key, teste.conta))
    }

    todos.sort_by(|a, b  | b.1.cmp(&a.1));
    for x in todos.iter() {
        println!("{} {}", x.1, x.0);
    }
}
