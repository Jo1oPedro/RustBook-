use std::cmp::Ordering;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Pessoa {
    nome: String,
    idade: u32,
}

fn meu_cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a.is_nan() {
        return Ordering::Less;
    }
    if b.is_nan() {
        return Ordering::Greater;
    }
    if a < b {
        return Ordering::Less;
    }
    if a > b {
        return Ordering::Greater;
    }

    Ordering::Equal
}

fn main() {
    println!("Hello, world!");

    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(3);
    numeros.push(7);
    numeros.push(6);
    numeros.push(2);
    numeros.push(1);

    println!("Inteiros original");
    println!("{:?}", numeros);

    println!("\n Inteiros ordem natural");
    numeros.sort();
    numeros.sort_unstable();
    println!("{:?}", numeros);

    println!("\n Inteiros ordem natural reversa");
    numeros.reverse();
    println!("{:?}", numeros);

    // 2  floats
    let mut floats: Vec<f64> = Vec::new();
    floats.push(3.3);
    floats.push(7.7);
    floats.push(6.6);
    floats.push(2.2);
    floats.push(1.1);
    floats.push(4.4);
    floats.push(5.5);

    println!("\n\n Floats originais");
    println!("{:?}", floats);

    println!("\n Floats sem NAN");

    let x = 9.9;
    let y = f64::NAN;

    if x > y {
        println!("9.9 é maior que NAN");
    } else if x < y {
        println!("9.9 é menor que NAN");
    } else if x == y {
        println!("9.9 é igual a NAN");
    } else {
        println!("sei lá");
    }

    //floats.push(f64::NAN);
    floats.sort_by(|a, b| a.partial_cmp(b).expect("Não pode ter NAN!!!"));
    println!("{:?}", floats);

    println!("\n Floats com NAN");
    floats[5] = f64::NAN;
    println!("{:?}", floats);
    floats.sort_by(|a, b| meu_cmp_f64(a, b));
    println!("{:?}", floats);

    // semantica move;
    let mut populacoes: Vec<(String,String)> = Vec::new();
    populacoes.push( ("Wenceslau Braz".to_string(),"PR".to_string()) );
    populacoes.push( ("Wenceslau Braz".to_string(),"MG".to_string()) );
    populacoes.push( ("São Carlos".to_string(),"SC".to_string()) );
    populacoes.push( ("São Carlos".to_string(),"SP".to_string()) );
    populacoes.push( ("São Domingos".to_string(),"GO".to_string()) );
    populacoes.push( ("São Domingos".to_string(),"SC".to_string()) );
    populacoes.push( ("São Domingos".to_string(),"BA".to_string()) );
    populacoes.push( ("São Domingos".to_string(),"SE".to_string()) );
    populacoes.push( ("São Francisco".to_string(),"PB".to_string()) );
    populacoes.push( ("São Francisco".to_string(),"SP".to_string()) );
    populacoes.push( ("São Francisco".to_string(),"MG".to_string()) );
    populacoes.push( ("São Francisco".to_string(),"SE".to_string()) );
    populacoes.push( ("São Francisco de Paula".to_string(),"RS".to_string()) );
    populacoes.push( ("São Francisco de Paula".to_string(),"MG".to_string()) );

    for x in populacoes.iter() {
        println!("{:?}", x);
    }

    println!("Tuplas ordenação natural: usa os elementos da tupla da esquerda para a direita");
    populacoes.sort();
    for x in populacoes.iter() {
        println!("{:?}", x);
    }

    println!("Tuplas ordenação pelos estados: usa elementos da tupla da *direita* para a esquerda");
    populacoes.sort_by(|a, b| if a.1 != b.1 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

    for x in populacoes.iter() {
        println!("{:?}", x);
    }

    // semantica copy
    let mut numeros: Vec<(i32, i32)> = Vec::new();
    numeros.push( (3,555) );
    numeros.push( (7,444) );
    numeros.push( (6,111) );
    numeros.push( (2,222) );
    numeros.push( (1,666) );
    numeros.push( (4,333) );
    numeros.push( (5,777) );

    println!("Tuplas com semântica copy: cria nova tupla com a ordem certa para ordernar");
    numeros.sort_by_key(|t| (t.1, t.0));
    for x in numeros.iter() {
        println!("{:?}", x);
    }

    
}
