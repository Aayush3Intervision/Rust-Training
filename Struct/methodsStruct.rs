struct Reactangle{
    height:usize,
    width:usize,
}

impl Reactangle{
    fn area(&self)-> usize {
        self.height * self.width
    }
}

fn main(){
    let calculate = Reactangle{
        height: 10,
        width: 33,
    };
    println!("height is {}, width is {} and area is {}",calculate.height, calculate.width, calculate.area());
}