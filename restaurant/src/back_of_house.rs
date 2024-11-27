fn fix_incorrect_order(){
    cook_order(); // This function is directly accessable because it is defined in the same scope/
    super::deliver_order(); // super keyword is used for get    ting the parent module to be available in this scope.
    // crate::deliver_order();
}
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
fn cook_order(){}

/* *************** public(pub) Enum ************************* */
// In the struct the field which we want to use outside the module we have to make them public all those fields we needed but in the enum 
// we do not need to make every needed fields public we only need to make enum public then all fields become public.
pub enum Appetizer {
    Soup,
    Salad,
}
