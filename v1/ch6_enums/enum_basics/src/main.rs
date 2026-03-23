enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    if ip_kind == IpAddrKind::V4 {
        println!("This is IP_V4");
    }
    else{
        println!("This is IP_V6");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);


}
