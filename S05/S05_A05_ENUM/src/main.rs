#[derive(Debug)]
enum TipoEndIp {
    V4,
    V6
}

fn exemplo_enum_simples() {
    let tipo1 = TipoEndIp::V4;
    let tipo2: TipoEndIp;

    tipo2 = TipoEndIp::V6;

    println!("Exemplo enum simples");
    println!("tipo1 = {:?} tipo2 = {:?}", tipo1, tipo2);
}

#[derive(Debug)]
struct EndIp {
    tipo: TipoEndIp,
    endereco: String
}

fn exemplo_com_struct() {
    let home = EndIp {
        tipo: TipoEndIp::V4,
        endereco: String::from("home")
    };

    let loopback = EndIp {
        tipo: TipoEndIp::V6,
        endereco: String::from("::1")
    };

    println!("Exemplo com struct");
    println!("home {:?} loopback {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn exemplo_com_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Exemplo com enum");
    println!("home {:?} loopback {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddrDif {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn exemplo_com_enum2() {
    let home = IpAddrDif::V4(127, 0, 0, 1);
    let loopback = IpAddrDif::V6(String::from("::1"));

    println!("Exemplo com enum");
    println!("home {:?} loopback {:?}", home, loopback);
}

fn main() {
    exemplo_enum_simples();
}
