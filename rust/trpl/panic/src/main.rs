use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // v[99]; <- automatic panic - accessing an element out of bounds

    // This panics if the file does not exist or returns the file if it does
    let greeting_file = File::open("hello.txt").unwrap();
    let greetings_file = File::open("hello.txt").expect("Failed to open hello.txt");

    let greetings_file_result = File::open("hello.txt");

    let greetings_file = match greetings_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", error),
        },
    };

    error_handling_v2();
    read_username_from_file_v1();
}

fn error_handling_v2() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_username_from_file_v1() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    // No return in these legs here, because it's a function and just returns the result.
    // That's why we don't have ; either at the end.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_v2() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
