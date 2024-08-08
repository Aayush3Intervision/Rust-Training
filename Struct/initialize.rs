struct Book{
    book_name:String,
    book_author: String,
    publish_year: u32,
    book_price: usize,
    is_bestseller: bool
}

fn main(){
    let book = Book{
        book_name: String::from("Geeta"),
        book_author: String:: from("Lord Krishna"),
        book_price: 10000,
        publish_year: 0001,
        is_bestseller: true,
    };
    println!("The book name is as follows: {}",book.book_name);
    println!("The book author is: {}",book.book_author);
    println!("The book is published in: {}",book.publish_year);
    println!("The book is best seller: {}",book.is_bestseller);
}