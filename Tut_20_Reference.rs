fn main() {
    let mut a:usize = 23;
    // let mut b: &usize = &mut a;
    {
        let c = &mut a;
        *c += 1;
        println!("The value of is {c}");
    }
    // println!("The value of is {b}");

    println!("The value of is {a}");
}