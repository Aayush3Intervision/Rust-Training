fn main(){
    test_while_loop();
}

fn test_while_loop(){
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter +=1;
    }
    println!("Counter {}", counter);
}