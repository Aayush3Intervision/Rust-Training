struct Square {
    val_1:usize,
}

impl Square{
    fn squaring(&self)->usize{
        return self.val_1 * self.val_1;
    }
}

fn main(){
    let result = Square{
        val_1: 10   
    };
    println!("result of value {} square is: {}",result.val_1,result.squaring());

    let value = Table{
        table_of:9,
    };
    println!("Result from table {}",value.table());

}

struct Table{
    table_of: usize,
}
impl Table {
    fn table( &self ) -> usize{
        let mut result = 0;
        for i in 1..11 {
            result = self.table_of * i;
            println!("{}",result);
        }
        return result;
    }
}