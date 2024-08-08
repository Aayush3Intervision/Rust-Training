fn main(){
    let var_1: f64 = 1234.5;
    println!("Variable 1 is {}",var_1);
    var_1 = 1234.0;
    println!("value changed {}",var_1);
}
/*
This file gives error because by-default rust variables are immutable,
so reinitialize the value in predifined variables is not possible we have to make the mutable.
*/