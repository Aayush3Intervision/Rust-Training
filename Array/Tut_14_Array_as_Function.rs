fn main(){
    let arr = [1,2,3,4];
    update(arr);
    println!("Inside main function");
}

fn update(mut arr:[usize;4]){
    println!("Inside the update function");
   for i in 0..4{
    arr[i] = 0;
   }
   println!("arr {:?}",arr);
}