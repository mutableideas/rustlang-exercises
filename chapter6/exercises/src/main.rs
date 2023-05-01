#[derive(Debug)]
enum IPKind {
    V4,
    V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
struct IPAddress {
    address: String,
    kind: IPKind
}

impl IPAddress {
    fn from(address: String) -> IPAddress {
        IPAddress { address, kind: IPKind::V4 }
    }
}

// struct enum
struct IpV6 {
    address: String
}

struct IpV4 {
    address: String
}

enum IpAddrStruct {
    V4(IpV4),
    V6(IpV6)
}

#[derive(Debug)]
enum UsState {
    Michigan,
    NorthCarolina
}

// enum with match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    HalfDollar,
    Dollar
}

fn main() {
    let ip_address = String::from("127.0.0.1");
    let address = IPAddress::from(ip_address);

    println!("IP Address {:#?}", address);

    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let add_one = plus_one(Some(6));
    let add_none = plus_one(None);

    println!("Add one, add none {:?} {:?}", add_one, add_none);

    let coin = Coin::Penny;

    if let Coin::Penny = coin {
        println!("Dont need to deal with place holder with if let")
    }

    let cents = value_in_cents(Coin::Quarter(UsState::Michigan));
    println!("Cent {}", cents);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is {:?}!", state);
            25
        },
        Coin::HalfDollar => 50,
        Coin::Dollar => 100
    }
}

fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
      None => None,
      Some(i) => Some(i + 1)
    }
}

fn placeholder_add(option: u8) {
    match option {
        1 => println!("hi 1"),
        2 => println!("Hi 2"),
        _ => println!("Value not in match")
    }
}
