fn main (){
    let a = returns();
    println!("The return value is: {}",a);
    let b = 3;
    let c = return_with_out_semi_colon(b);
    println!("The value of c is: {}",c);
}

fn returns() -> usize{
    let a:usize = 120;
    // println!("The value of a is: {}",a);
    return a;
}

// If any statement specify return data type and use variable without semi colon that means it will be return that variable,
fn return_with_out_semi_colon(param: usize) -> usize{
    param * 3
} 