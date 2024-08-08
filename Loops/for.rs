fn main(){
    let result = test_for_loop(8);
    println!("result {}",result);
}

fn test_for_loop( number:isize ) -> isize{
    let mut table=0;
    for i in 1..11 {
        table = number*i;
        println!("{} X {} = {}", number,i,table);
    }
    return table;
}