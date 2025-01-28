// Prevent the compiler from warning about unused code
#![allow(unused)] 

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */

enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    */

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let v4_v2 = IpAddrV2::V4(127, 0, 0, 1);
    let v6_v2 = IpAddrV2::V6(String::from("::1"));
    println!("V6: {:?}", v6_v2);

    let coin: Coin = Coin::Quarter(UsState::Washington);
    let coin_value: u8 = value_in_cents(coin);
    println!("Coin value: {}", coin_value);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => reroll(),
        // _ => (),
    }

}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to {:?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}