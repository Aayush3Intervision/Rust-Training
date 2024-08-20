struct Addition{
    a:usize,
    b:usize,
}

impl Addition{
    fn initialize(param1: usize, param2:usize)-> Addition{
        Addition{a:param1, b:param2}
    }

    fn add(&self) {
        let c = self.a + self.b;
        println!("a = {}, b = {} and addition is: {}",self.a, self.b, c);
    }
}

fn main(){
    let result = Addition::initialize(2,88);
    // println!("{}",result.add());
    result.add();
}