
fn add_fancy_hat() {
    println!("Fancy hat");
}

fn remove_fancy_hat() {
    println!("Remove fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("Move player {num_spaces}");
}

fn reroll() {
    println!("Reroll");
}

fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    }

    let nada = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    };

    println!("{:?}", nada);
}
