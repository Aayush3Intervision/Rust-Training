// enum IpAddrKind {
//     V4,
//     V6
// }

// struct Initialize {
//     kind: IPAddrKind,
//     address: String
// }

// fn main(){
//     let ip = Initilize {
//         kind: IpAddrKind::V4,
//         address: String::from("192:08:001")
//     };
//     println!("ip {}",ip);
// }
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main(){
    let ip = IpAddrKind::V4(String::from("127:99:2291"));
    println!("ip {:?}", ip);
}

