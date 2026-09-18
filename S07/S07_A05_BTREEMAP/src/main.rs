use std::collections::{HashMap, BTreeMap};

fn main() {
    let mut turma_hash = HashMap::new();
    let mut turma_btree = BTreeMap::new();

    let nomes = vec![
        "Ana", "Beatriz", "Claudio", "Daniel", "Ernesto", "Flávia",
        "Geraldo", "Rômulo", "Remo", "Saionara"
    ];

    let notas = vec![0,1,2,3,4,5,6,7,8,9];

    for i in 0..10 {
        turma_hash.insert(nomes[i], notas[i]);
        turma_btree.insert(nomes[i], notas[i]);
    }

    println!("Pesquisa");
    let aluno = "Daniel";

    match turma_hash.get(&aluno) {
        Some(n) => println!("HashMap tem {} com nota {}", aluno, n),
        None => println!("HashMap não tem nota para {}", aluno)
    }

    match turma_btree.get(&aluno) {
        Some(n) => println!("HashMap tem {} com nota {}", aluno, n),
        None => println!("HashMap não tem nota para {}", aluno)
    }

    println!("Iteração com HashMap");
    for(nome, nota) in &turma_hash {
        println!("Iterando --> {} --> {}", nome, nota);
    }

    println!("Iteração com BTreeMap");
    for(nome, nota) in &turma_btree {
        println!("Iterando --> {} --> {}", nome, nota);
    }

    println!("Iteracao com BtreeMap e um intervalo");
    let intervalo = turma_btree.range("R"..="S");

    for(nome, nota) in intervalo {
        println!("Iterando --> {} --> {}", nome, nota);
    }
}
