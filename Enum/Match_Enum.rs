fn main(){
    fn plus_one(x: Option<i32>)->Option<i32>{
        match x {
            None => {
                println!("None returned");
                None
            },
            Some(i) => {
                println!("Value is: {}",i);
                Some(i+1)
            },
        }
    }
    let a = Some(6);
    let b = plus_one(a);
    let c = plus_one(None);
}