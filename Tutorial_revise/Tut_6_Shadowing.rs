fn main(){
    let name:&str = "Aayush";
    println!("Hello, {}!",name);
    let name:&str = "AK";
    println!("Shadowing var name with:{}",name);
    let mut name:usize = name.len();
    println!("Shadowing name with change in data type from string to integer: {}",name);
    name = 5;
    println!("change again var name length with:{}",name);
}