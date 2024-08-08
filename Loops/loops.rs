fn main(){
    let result:usize = test_loops(2);
    println!("result {}",result);

}

fn test_loops (a: usize) -> usize {
    let mut b;
    let mut i = 0;
    loop {
        i+=1;
        b = a*i;
        println!("Table of {a} is {a} X {i} = {b} ");
        if i==10{
            break;
        }
    }
    return b;
}