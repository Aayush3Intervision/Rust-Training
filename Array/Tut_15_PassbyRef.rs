fn main (){
    let mut x:[usize; 2] = [1,2];
    println!("Inside main");
    update(&mut x);
}

fn update (arr: &mut[usize;2]){
    println!("Inside update");
    for i in 0..2{
        arr[i] = arr[i] + 1;
    }
    println!("arr {:?}",arr);
}