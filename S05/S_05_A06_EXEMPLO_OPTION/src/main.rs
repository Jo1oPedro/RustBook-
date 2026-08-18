use std::io;
enum TipoAluno {
    NaoBolsista,
    Bolsista(u32),
}

struct Aluno {
    nome: String,
    nota: f64,
    tipo: TipoAluno
}

fn consulta(nome: &str, turma: &Vec<Aluno>) -> Option<f64> {
    for a in turma {
        if nome == a.nome {
            return Some(a.nota);
        }
    }
    None
}

fn main() {
    let turma = vec![
        Aluno{nome: "Joao".to_string(), nota: 7.8, tipo: TipoAluno::NaoBolsista},
        Aluno{nome: "Maria".to_string(), nota: 7.8, tipo: TipoAluno::Bolsista(60)},
    ];

    match turma[1].tipo {
        TipoAluno::NaoBolsista => println!("{} Não é bolsista", turma[1].nome),
        TipoAluno::Bolsista(number) => println!("{} tem bolsa de {}%", turma[1].nome, number),
    }

    loop {
        println!("Digite o nome: ");
        let mut linha = String::new();
        io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");

        let nome_option = linha.trim().split_whitespace().nth(0);

        match nome_option {
            Some(x) => println!("Some: Leu {}", x),
            None => println!("Nome: Não leu")
        }

        if let Some(nome) = nome_option {
            println!("Nome: {}", nome);

            if let Some(nota) = consulta(nome, &turma) {
                println!("Tem nota {}", nota);
                match nota {
                    x if x < 5.0 => println!("Está reprovado!"),
                    x if x < 7.0 => println!("Está em recuperação!"),
                    x if x <= 10.0 => println!("Está aprovado!"),
                    _ => panic!("Nota com valor inesperado: {}", nota)
                }
            } else {
                println!("Aluno desconhecido");
            }
        } else {
            println!("Nenhum nome foi teclado!");
            break;
        }
    }
}
