use std::thread::sleep;
use std::time::Duration;

const _VIAH_MARGEM: f64 = 15.0; // metros
const _VIAV_MARGEM: f64 = 15.0; // metros

const VIAH_LARGURA: f64 = 4.0; // metros
const VIAV_LARGURA: f64 = 4.0; // metros

const _VIAH_PERIMETRO: f64 = 150.0; // metros
const _VIAV_PERIMETRO: f64 = 150.0; // metros

const CARRO_LARGURA: f64 = 2.0; // metros
const CARRO_COMPRIMENTO: f64 = 4.0; // metros

const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

const ACELERACAO_MAXIMA: f64 = 3.0;

const ACELERACAO_MINIMA: f64 = -10.0;

// Simula 2 carros até saírem do perímetro controaldo ou coliderem
// Retorna se houve colisão ou não
fn simula_carros(via_carro1: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64) -> bool {
    // Carro 1
    let chassi1: i32 = 1111; // identificação de um carro
    let via1: char = via_carro1; // via deste carro
    let _acel_max1 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado
    let _acel_min1 = ACELERACAO_MINIMA; // metros por segundo ao quadrado
    let vel_max1 = VELOCIDADE_MAXIMA; // metros por segundo
    let comprimento1 = CARRO_COMPRIMENTO; // metros
    let mut pos_atual1: f64 = -80.0; // metros do cruzamento
    let mut vel_atual1: f64 = 0.0; // metros por segundo
    let acel_atual1: f64; // metros por segundo ao quadrado


    // Carro 2
    let chassi2: i32 = 2222; // identificação de um carro
    let via2: char = via_carro2; // via deste carro
    let _acel_max2 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado
    let _acel_min2 = ACELERACAO_MINIMA; // metros por segundo ao quadrado
    let vel_max2 = VELOCIDADE_MAXIMA; // metros por segundo
    let comprimento2 = CARRO_COMPRIMENTO; // metros
    let mut pos_atual2: f64 = -100.0; // metros do cruzamento
    let mut vel_atual2: f64 = 0.0; // metros por segundo
    let acel_atual2: f64; // metros por segundo ao quadrado

    acel_atual1 = acel_atual1;
    acel_atual2 = acel_atual2;

    println!("Inicio da simulação");
    let mut tickms: f64;

    loop {
        sleep(Duration::from_millis(100));
        tickms = 100.0;

        let old_position = pos_atual1;

        pos_atual1 = pos_atual1 + vel_atual1 * ( tickms / 1000.0 )  + acel_atual1 * (tickms/1000.0) * (tickms/1000.0) / 2.0;
        vel_atual1 = vel_atual1 + acel_atual1 * ( tickms / 1000.0 );

        if pos_atual1 < old_position {
            pos_atual1 = old_position;
        }
    };
}

fn main() {
    println!("Hello, world!");
}
