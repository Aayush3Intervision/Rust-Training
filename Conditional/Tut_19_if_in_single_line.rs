fn main(){
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is:{}",number);

    let condition_1 = false;
    let num2 = if condition_1 { 11 } else { 4 };
    println!("number is: {}", num2);
}