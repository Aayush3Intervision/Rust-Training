fn main(){
    let arr:[i32;4] = [1,2,3,4];
    println!("Array is {:?}", arr);
    println!("Array size is : {}",arr.len());

    for value in arr.iter(){
        println!("Value is {}",value);
    }
}