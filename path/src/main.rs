use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// // It is fine to use the 'use' to import other modules into the scope but the problem is with the same name 
// // If some other module with the same name brings into the scope, it is not allows by rust.
// use std::fmt;
// use std::io;

// fn function1()-> fmt::result{
//     //some code
// }

// fn function2()-> io::result{
//     //some code
// }

// As we can see the above two functions are trying to return different types with the same name. This is not allowed in
// the rust. So as the result we can provide new name to one of the functions or both.

/********************* Provinding new name with "as" keyword ****************************/
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     //code
// } 
// fn function2() -> IoResult<()> {
//     //code
// }

/************************ External dependencies *******************************/
use rand::Rng;
fn useRand() {
    let secret_number= rand::thread_rng().gen_range(1..=100);
}

/*********************** Nested Path and Clean large "use" list ***********************************/
// use std::cmp::Ordering;
// use std::io;
// Instead of writing this we can write the following code as 

// use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
// we can import the above code with the following way
use std::{self, io::Write}; //Self is used for for importing std::io module too.

/***************** Global operator **********************/
// If we wanted to import the all the files from the std module or any third party module we can use "*" astric operator
// for that as 
use::std::collection::*; 
