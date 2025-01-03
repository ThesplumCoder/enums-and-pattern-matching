#[derive(Debug)]
enum IpAddress {
    V4 { address: String, ip_type: IpTypes },
    V6,
}

#[derive(Debug)]
enum IpTypes {
    Loopback,
    Private,
    Public,
    Broadcast,
}

impl IpAddress {
    fn whoami(&self) {
        println!("I'm {:?}", self);
    }
}

fn main() {
    let myhouse = IpAddress::V4 {
        address: String::from("221.211.103.093"),
        ip_type: IpTypes::Public,
    };
    println!("Examples of enums running.");
    myhouse.whoami();
}
