fn main(){
    let a = (500, 2,4,"Aayush");
    println!("Values of tuple is:{},{},{}",a.1,a.2,a.3);
    let b = (1,2,"str");
    let (x,y,z) = b;
    println!("value of tuple variable is: {},{}, and, {}",x,y,z);
}