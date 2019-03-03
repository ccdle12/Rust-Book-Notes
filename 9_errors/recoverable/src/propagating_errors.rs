use std::io;
use std::io::Read;
use std::fs::File;

// read_username_from_file will attempt to read a username from a file. 
// The return value replaces the generic types <T,E> with <String, io:Error>
pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// read_username_from_file will attempt to read a username from a file. 
// The return value replaces the generic types <T,E> with <String, io:Error>
pub fn read_username_from_file_2() -> Result<String, io::Error> {
    // ? is a short way to repeat the match statement,
    // it will either return the value insisde of Ok, or the Error.
    // let mut f = File::open("hello.txt")?;

    // Create a string, assign the value of the error or the ok value to the string, return the
    // string.
    let mut s = String::new();

    // Read the values of f to the s. Return an Ok value or Error value.
    // f.read_to_string(&mut s)?;
    // Ok(s)
    
    // Even shorter way, by chaining the calls.
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}
