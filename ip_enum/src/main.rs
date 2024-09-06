enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}

fn route(ip_kind: IpAddr) {
    match ip_kind {
        IpAddr::V4(_, _, _, _) => println!("IPv4"),
        IpAddr::V6(_) => println!("IPv6"),
    }
}
