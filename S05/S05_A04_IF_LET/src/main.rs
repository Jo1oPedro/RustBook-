enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usize)
}

#[derive(Debug)]
enum Mensagem {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {
    let config_max = Some(3);

    match config_max {
        Some(max) => println!("Match, the maximum is configured to be {}", max),
        _ => ()
    }

    if let Some(max) = config_max {
        println!("if_let, The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(1999);
    let mut count = 0;

    match coin {
        Coin::Quarter(ano) => println!("match, Ano do quarter {:?}!", ano),
        _ => count += 1
    }

    if let Coin::Quarter(ano) = coin {
        println!("if_let, Ano do quarter {:?}!", ano);
    } else {
        count += 1;
    }

    let m1 = Mensagem::Write(String::from("hello"));
    let m2 = Mensagem::ChangeColor(0, 255, 255);
    if let Mensagem::Write(txt) = m1 {
        println!("{}", txt);
    }
    if let Mensagem::Write(txt) = m2 {
        println!("O texto no Write m2 é {}", txt);
    }

    let m3 = Mensagem::Write(String::from("hello"));

    match m3 {
        Mensagem::Write(ref txt) => println!("{}", txt),
        _ => ()
    }

    let m3 = Mensagem::Write(String::from("hello"));

    if let Mensagem::Write(ref txt) = m3 {
        println!("{}", txt);
    }

    println!("M3 é: {:?}", m3);
}
