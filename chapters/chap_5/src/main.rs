// ENUMS AND PATTERN MATCHING
// Enums are types which have a few definite values
// Enums are useful whenever you have a set of values that you know will never change

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum _Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl _Message {
    fn _call(&self) {
        println!("Message called");
    }
}

// struct _IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }



// Option Enum
enum _Option<T> {
    _Some(T),
    _None,
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _localhost = IpAddrKind::V4(127, 0, 0, 1);

    // let localhost = _IpAddr {
    //     kind: _four,
    //     address: String::from("127.0.0.1"),
    // };

    route(IpAddrKind::V4(127, 0, 0, 1));





    // The Option Enum and Its Advantages Over Null Values
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("{}", sum);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
