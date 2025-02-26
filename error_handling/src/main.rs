fn main() {
    // panic!("Error panic code crash here");
    // let vec = vec![1, 2, 3];
    // vec[90];

    // recoverable errors
    // result enum 
    // enum Result<T,E>{
    //     OK(T),
    //     Err(E),
    // }

    // Lets look at the example that returns result enum.

    use std::fs::File;
    use std::io::ErrorKind;
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file)=> file,
        // Err(error) => panic!("Problem in opening the file:{error:?}"),

        // Matching with different errors.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc)=> fc,
                Err(e)=> panic!("Problem in creating the file:{e:?}")
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        }
        };
        // Instead of matching the kinds we can use clouser name unwrap_or_else.

        let greeting_file1 = File::open("new.txt").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound{
                File::create("new.txt").unwrap_or_else(|error|{
                    panic!("Problem in creating file:{error:?}");
                })
            }
            else{
                panic!("Problem opening the file: {error:?}")
            }
        });

        // let greeting_file2 = File::open("hello1.txt").unwrap();
        // instead of getting rust normal error we can do the same thing with expect method with beautifull error message.
        let greeing_file3 = File::open("hello.txt").expect("hello.txt file is not present in the repo.");

    // Propagating Errors
    use std::io::{self, Read};
    fn read_username_from_file()-> Result<String, io::Error>{
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result{
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();

        match username_file.read_to_string(&mut username){
            Ok(_)=> Ok(username),
            Err(e)=> Err(e),
        }
    }

    // A shortcut for propagating errors using ? operator.
    // the implementation we did just before we can do that same using ? operator like as follows
    fn read_otheruser_from_file()-> Result<String, io::Error>{
        let mut other_username_file_result = File::open("hello.txt")?;
        let mut other_username = String::new();
        other_username_file_result.read_to_string(&mut other_username)?;
        Ok(other_username)
    }
    // we can write the same code in much less code as
    fn read_file()-> Result<String, io::Error>{
        let mut user_name = String::new();
        File::open("hello.txt")?.read_to_string(&mut user_name)?;
        Ok(user_name)
    }

    // instead of opening new file then create and after it read the file we can do this in simple ways as
    use std::fs;
    use std::io;

    fn read_username_from_file()-> Result<String, io::Error>{
        fs::read_to_string("hello.txt")
    } 
}
