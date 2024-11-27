mod front_of_house;

/* Using "use" keyword to bring all the function of hosting sub-module into the crate level scope, after this, 
user will not need to define repeated path for sub-module.
*/
// use crate::front_of_house::hosting; 
pub use crate::front_of_house::hosting; // It is also known as re-exporting. Making our module available for external code to use.
//we can then make our module path public using "pub" keyword.
// For other files which import that module to use add_to_waitlist() will follows the way to do so as 
// restaurant::front_of_house::add_to_waitlist();

 
pub fn eat_at_restaurant(){
    /*  Path: path are the tree by using path we can brings other modules, functions, files etc into the scope.
        we have two ways to define path in rust 
        1. Absolute path: It is the path that starts with the root directory file also called crate in this case lib.rs is root create.
            ex. crate::front_of_house::hosting::add_to_waitlist()
        2. Relative path: It is the path that starts with the current module file
            ex. front_of_house::hosting::add_to_waitlist()
    */

    /* Privacy(public and private)
        If we want to access the modules from this function we have to mark the as public using pub keyword because 
        by-default everything is private in rust.
    
     */
    //Absolute path 
    crate::front_of_house::hosting::add_to_waitlist(); // When path is defined using "use" keyword the  there is not
    // need to define full path again and again only the function name is enough.

    // Relative Path
    front_of_house::hosting::add_to_waitlist();// When path is defined using "use" keyword the  there is not
    // need to define full path again and again only the function name is enough.

    hosting::add_to_waitlist(); // This is also correct way to bring the function of other module/crate into the scope
    // because we have defined the path using "use" keyword in the root crate.


    /* *************** public(pub) Struct ************************* */
    // order a breakfast in the summer with sandwitch
    let mut meal = back_of_house::Breakfast::summer("sandwitch");
    // Change our mind about what bread we would like
    meal.toast = String::from("wheat");
    println!("I would like {} toast please",meal.toast);

    // The next line won't compile if we uncomment it; we are not allowed
    // to see or modify the seasonal_fruit that comes with the meal 
    // meal.seasonal_fruit = String::from("blueberries");

    /* *************** public(pub) Enum ************************* */
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


/* Relative Path with super keyword */
// function deliver_order defines in the create module or root crate if any other module want to use this function
// in the same crate then we can use super keyword to access it as the alternet way of Relative path or absolute path.
fn deliver_order(){}

mod back_of_house;
/* This code gives error because when we are directly using modules and functions of front_Of_house module because for customer module, front_Of_house module sub-module is unknown. */
// mod customer {
//     pub fn eat_at_restaurent(){
//         hosting::add_to_waitlist();
//     }
// }