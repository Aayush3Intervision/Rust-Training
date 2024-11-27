/* Creating Struct public with module(mod) */
// As we know all the fields in the struct are private so we have to make them public and also values in the struct 
// are private so to use them we have to make also them public.
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast{
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit:String::from("Mango Juice"),
        }
    }
}
    