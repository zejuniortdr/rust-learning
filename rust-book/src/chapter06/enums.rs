use crate::utils::terminal::wait_for_enter;


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddr2 {
    kind: IpAddrKind,
    address: String,
}


enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


pub fn run() {

    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    describe_ip_kind(&four);
    describe_ip_kind(&six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    match home {
        IpAddr::V4(address) => println!("Home (IPv4): {}", address),
        IpAddr::V6(address) => println!("Home (IPv6): {}", address),
    }
    match loopback {
        IpAddr::V4(address) => println!("Home (IPv4): {}", address),
        IpAddr::V6(address) => println!("Home (IPv6): {}", address),
    }

    let home2 = IpAddr2 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback2 = IpAddr2 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("Home2 kind: {:?}, address: {}", home2.kind, home2.address);
    println!("Loopback2 kind: {:?}, address: {}\n", loopback2.kind, loopback2.address);

    let home3 = IpAddr3::V4(127,0,0,1);
    let loopback3 = IpAddr3::V6(String::from("127.0.0.1"));
    print_ip(home3);
    print_ip(loopback3);

    wait_for_enter();
}


fn print_ip(addr: IpAddr3) {
    match addr {
        IpAddr3::V4(a, b, c, d) => {
            println!("IPv4 Address: {}.{}.{}.{}\n", a, b, c, d);
        }
        IpAddr3::V6(addr) => {
            println!("IPv6 Address: {}\n", addr);
        }
    }
}

fn describe_ip_kind(kind: &IpAddrKind) {
    match kind {
        IpAddrKind::V4 => println!("This is an IPv4 address."),
        IpAddrKind::V6 => println!("This is an IPv6 address."),
    }
}
