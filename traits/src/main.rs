// use aggregator::{Summary, Tweet};

use std::{clone, fmt::{format, Debug, Display}};

fn main(){
    // initialize traits
    let article = NewsArticle{
        headline: "Todays gold price".to_string(),
        location: "indore".to_string(),
        author:"aayush".to_string(),
        content:"Today gold rate hit life time high".to_string(),
    };

    // initialize trait for tweet.
    let tweet = Tweet{
        retweet: true,
        reply:false,
        content: "Bitcoin crashed by the 50% from lifetime high.".to_string(),
        user_name:"Aayush".to_string(),
    };
    println!("{}",article.author);
    println!("1 new retweet: {}",tweet.summarize());
    // calling the notify function here
    notify(&article);
}
pub trait Summary{
    // fn summarize(&self)-> String; //We can also define default body in the summarize to ensure the user should get some
    // message in case of some failure like that. 
    fn summarize(&self)-> String{
        format!("(Read more form {}.....)",self.summarize_author())
    }
    // We can also define the multiple methods in the summary trait like
    fn summarize_author(&self)-> String;
}

// create another trait name display.
// pub trait Display
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// lets implement traits for summary trait
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // we need to define every method which is part of trait because we are implement whole trait here.
    fn summarize_author(&self)-> String {
        format!("{}",self.author)
    }
} 

pub struct Tweet{
    pub reply:bool,
    pub retweet: bool,
    pub user_name: String,
    pub content: String,
}
// define summary trait for tweet
impl Summary for Tweet{
    fn summarize(&self)-> String {
        format!("{}, {}",self.user_name, self.retweet)
    }
    // we need to define every method which is part of trait because we are implement whole trait here.
    fn summarize_author(&self)-> String {
        format!("@{}",self.user_name)
    }
}  

// // we can take trait as a parameter also as
// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}",item.summarize());
// }

// Above code is syntax sugar for trait bound.
pub fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}",item.summarize());
}
// Some times we can have two parameters that implement summary.
// pub fn notify_1(item1: &impl Summary, item2: &impl Summary){} 

// we can implement this two in the way like this.
pub fn notify_1<T: Summary>(item1: &T, item2: &T){}

// We can use the multiple trait with the + syntax.
pub fn notify_2(item: &(impl Summary + Display)){}

// + syntax is also valid for the generic type
pub fn notify_3<T: Summary + Display>(item: &T){}

// Making code more simple with the where clause because it is become complecated if we make the code with the multiple trait 
// so 
pub fn some_function<T: Display+Clone, U: Clone+ Debug>(t: &T, u: &U) -> i32 {
    56//simple for preventing error
} //instead of writing this we can use where clause

pub fn some_function_1<T, U>(t:&T, u: &U) -> i32
where 
    T: Display+Clone,
    U: Clone+Debug,
    {
        56//simple for preventing error
    }

// returning types that implement trait
// fn returns_summarizable() -> impl Summary{
//     Tweet{
//         user_name: "horse_ebooks".to_string(),
//         content: "of course, as you probably already know, people".to_string(),
//         reply: false,
//         retweet: false,
//     }
// }

// returning either newsArticle or a tweet is not allowed due to restriction around how the impl trait syntax is implemented
// by compiler. 
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            user_name: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        }
    }
}