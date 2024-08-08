struct User {
    name: String,
    account_age: u32,
    email: String,
    contact: usize,
    user_age: u32,
    gender: String,
    is_active: bool,
}

fn main() {
    let mut user = User{
        name: String::from("Aayush"),
        account_age: 10,
        email: String::from("aayush@gmail.com"),
        contact: 1223546780,
        user_age: 26,
        gender: String::from("Male"),
        is_active: true,
    };

    println!(
        "User name {}\n User account age is {}\n User email is {}\n User contact is {}\n is active {} ",user.name,user.account_age,user.email,user.contact, user.is_active
    );
    user.is_active = false;
    println!("Updated struct value is {}",user.is_active);
}
