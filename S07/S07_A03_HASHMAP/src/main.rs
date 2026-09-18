use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        None => println!("P1: get --> {team_name} não tem score"),
        Some(i) => println!("P1: get --> {team_name} tem score: {i}")
    }

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("P2: get(team_name) score --> {score}");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let nome_cor = String::from("red");
    let numero = 10;
    scores.insert(nome_cor.clone(), numero);

    println!("P5: nome_or: {nome_cor}");
    println!("P6: inseriu -> numero {numero}");

    scores.insert(String::from("Blue"), 25);
    println!("P7: {:?}", scores);

    let x = scores.entry(String::from("Yellow")).or_insert(90);
    let y = scores.entry(String::from("Black")).or_insert(999);
    println!("P8: {:?}", scores);

    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let ref_entrada = scores.entry(word.to_string()).or_insert(0);
        *ref_entrada += 1;
    }

    let valor = scores.get_mut(&String::from("Yellow"));
    match valor {
        None => (),
        Some(x) => *x += 100
    }
    println!("{:?}", scores);

    scores.remove("Red");
    println!("P11: remove --> {:?}", scores);
}
