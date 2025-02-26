// fn largest_number(list:&[i32] ) -> &i32{
//     let mut largest = &list[0];
//     for item in list{
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list:&[char] ) -> &char{
//     let mut largest = &list[0];
//     for item in list{
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

// the above two function finds largest value from the vector and the code inside is same so we can use generic type
// to reduce the code.

// let us implement for this code repetation and make code optimize.

fn largest_value<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
fn main(){
    let vect = vec![1,2,4,56,6];
    // let result = largest_number(&vect); // replace this function call we generic type functions
    let result = largest_value(&vect);
    println!("The largest number is:{}",result);

    let char_list = vec!['a','b','c','d'];
    // let char_result = largest_char(&char_list); // replace this function call we generic type functions
    let char_result = largest_value(&char_list);
    println!("The largest char is:{}",char_result);

    // calling struct
    // struct_with_generic();
    struct_with_gen();

    // struct implementation method used
    let p = Point {x:5, y:6};
    println!("p.x = {}",p.x());
}

// Struct with generic type can defined as followed using '<>'
// struct Point {
//     x: i32,
//     y: i32,
// }
// // we can only use single data type i32 with this approch let use generic for the use any data type

// struct Point<T> {
//     x: T,
//     Y: T,
// }
// for this we use different data type like for x -> i32 and for y -> f64 this will give error because of struct has 
// only T data type 
// to solve this issue we have to define struct with different type like as
struct Point<T, U>{
    x: T,
    y: U,
}
// Now if we store one value in i32 and one in f64 type we do not get error.

// fn struct_with_generic(){
//     let struct_initialze = Point{x:4, y:8};
//     println!("{0}",struct_initialze.x);
// }

// Redefine function for generic type
fn struct_with_gen(){
    let both_int_and_float = Point{ x:5, y:5.0};
    let both_int = Point{x:5, y:5};
    let both_float = Point{x:5.0, y:6.0};
}
// call this function in main we do not get error.

// enum with generic type
enum Option<T> {
    Some(T),
    None,
}

// We can also define enum with result enum as
enum Result<T, E>{
    Ok(T),
    Err(E),
}

// We can implement methods on struct and enum with the generic data type too as we have Point Struct with generic type 
// we can also do this with implement methods.
impl<T, U> Point<T, U>{
    fn x(&self)-> &T{
        &self.x
    }
}
