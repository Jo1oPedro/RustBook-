fn main() {
    let mut i = 0;

    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
        if i >= 10 {
            break;
        }
    }

    let result = loop {
        i += 100;
        if i >= 100 {
            break i * 2;
        }
    };
    println!("The result is {}", result);

    println!("Labels em loops");

    let mut contagem = 0;

    'meu_externo: loop {
        let mut faltam = 100;
        loop {
            if faltam == 97 {
                break;
            }

            if contagem == 2 {
                break 'meu_externo;
            }
            
            faltam -= 1;
        }
        contagem += 1;
    }
}
