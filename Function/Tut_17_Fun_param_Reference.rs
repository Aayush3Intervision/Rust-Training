fn main(){
    let mut no:usize = 2;
    let result = de_reference(&mut no);
    println!("Result: {}", result);
}

fn de_reference(param_no: &mut usize) -> usize{
    *param_no*2
}