fn somar_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn somar_option(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match(x, y) {
        (Some(i) , Some(j)) => Some(i * j),
        (Some(i), None) => None,
        (None, Some(j)) => None,
        (None, None) => None
    }
}

fn main() {
    let numero5 = Some(5);
    let nao_numero: Option<i32> = None;
    let some_char = Some('e');
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
}
