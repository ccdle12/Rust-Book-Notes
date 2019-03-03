use::std::fs::File;
use::std::io::ErrorKind;
mod propagating_errors;

fn main() {
    // We have inentionally given f the wrong file type. It will instead display the type
    // std::result::Result<std::fs::File, std::io::Error>
    // let f: u32 = File::open("hello.txt");
    
    // We can handle recoverable errors using the enum type `Result<T, E>`. 
    // T being the reutned value.
    // E being the error.
    // If reading the file was successful it will hold an instance of 
    // `Ok` = contains a file handle.
    //
    // If reading the file was not successful it will hold an instance of
    // `Err` = contains a message.
    //
    // If the match Ok was found, return the file and assign it to f.
    // If the match Err was found, panic!.
    // let f = File::open("hellworld.txt");
    // let f = match f {
        // Ok(file) => file,
        // Err(err) => {
            // panic!("There was a problem opening the file: {:?}", err)
        // },
    // };
    
    // Handling different errors.
    let f = File::open("hello.txt");

    // If an error is returned match the kind of error. If the error is `NotFound`, create the
    // file.
    // If there was an error creating the file, catch it else return other_error
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        },
    };

    // More concise way of writing the above.
    // Uses unwrap and closures. After Chapter 13 come back to this.
    let f = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        } 
    });

    // Unrwap is a helper method of `Result<T,E>`. If the variant returned is Ok it will return
    // the value. If the variant is Err, it will panic.
    // let f = File::open("hello_world.txt").unwrap();
    
    // Using expect will let us choose the return message.
    // let f = File::open("hello_world.txt").expect("Failed to open file hello_world.txt");
    // let s = propagating_errors::read_username_from_file();
    let s = propagating_errors::read_username_from_file_2();

    // ? Can ONLY be used in functions that return the Result<> enum.
    print!("{:?}", s);
}

