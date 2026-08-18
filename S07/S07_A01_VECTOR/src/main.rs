fn main() {
    let _vi1: Vec<i32> = Vec::new();
    let mut vi2: Vec<i32> = Vec::new();
    vi2.push(5);
    vi2.push(6);
    vi2.push(7);
    vi2.push(8);

    let _vi3 = vec![1, 2, 3, 4, 5];

    let _vss1: Vec<&str> = Vec::new();

    let mut vss2 = Vec::new();
    vss2.push("aaaa");
    vss2.push("bbbb");
    vss2.push("cccc");
    vss2.push("dddd");

    let _vss3 = vec!["aaaa", "bbbb", "cccc", "dddd"];

    let _vs1: Vec<String> = Vec::new();

    let mut vs2 = Vec::new();
    vs2.push(String::from("AAAA"));
    vs2.push(String::from("BBBB"));
    vs2.push(String::from("CCCC"));
    vs2.push(String::from("DDDD"));

    let _vs3 = vec![String::from("aaaa"), String::from("bbbb"), String::from("cccc"), String::from("dddd")];

    let x = vi2[2];
    let y = vss2[2];
    //let z = vs2[2];

    let xx = &vi2[2];
    let yy = &vss2[2];
    let zz = &vs2[2];

    let xxx = vi2.get(2);
    let yyy = vss2.get(2);
    let zzz = vs2.get(2);

    match xxx {
        Some(valor) => println!("xxx {valor}"),
        None => println!("Não existe xxx")
    }

    match yyy {
        Some(valor) => println!("yyy {valor}"),
        None => println!("Não existe yyy")
    }

    match zzz {
        Some(valor) => println!("zzz {valor}"),
        None => println!("Não existe zzz")
    }

    let mut v4 = vec![1, 2, 3, 4, 5];

    let first = &v4[0];

    //v4.push(1);

    println!("The first element is {}", first);

    v4.push(26);


    // ITERACAO

    println!("Iterar sem alterar");

    for i in &vs2 {
        println!("{}", i);
    }

    println!("Iterar alterando");
    for i in &mut vs2 {
        i.push_str("zz");
        println!("{}", i);
    }

    println!("Iterar alterando no caso de inteiros");
    println!("v4 antes de +9000 {:?}", v4);
    for i in &mut v4 {
        *i += 9000;
    }

    //Vector de enum
    #[derive(Debug)]
    enum Celula {
        MeuInteiro(i32),
        MeuFloat(f64),
        MeuTexto(String),
    }

    let mut linha = vec![
        Celula::MeuInteiro(3),
        Celula::MeuFloat(2.5),
        Celula::MeuTexto(String::from("hello world")),
    ];

    println!("Elemento 1 do vector linha é: {:?}", linha[1]);

    println!("Antes da ordenação--> {:?}", v4);
    v4.sort();
    v4.sort_unstable(); // Mais rapido, mas não preserva a ordem dos iguais
    println!("Depois da ordenação--> {:?}", v4)
}
