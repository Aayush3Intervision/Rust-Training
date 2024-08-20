#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6,
}

fn main(){
    let ip = IPAddrKind::V4;
    println!("ip {:?}",ip);
}