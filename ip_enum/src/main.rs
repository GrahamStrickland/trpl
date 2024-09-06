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
    // match ip_kind {
    //     | IpAddrKind::V4
    //         println!("127.0.0.1");
    //     | IpAddrKind::V6 {
    //     println!("2001:db8::8a2e:370:7334");
    // }
}
