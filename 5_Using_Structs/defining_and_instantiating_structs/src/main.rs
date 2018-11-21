// Create a struct.
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Create a User and assign it to a variable.
    let user1 = User {
        email: String::from("someone@somone.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Creating a mutable instance of a User.
    let mut user2 = User {
        email: String::from("someone@somone.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Update the email field of the User instance, since 
    // the variable is mutable.
    user2.email = String::from("updateduser@example.com");

    // Print the username email
    println!("user2 email: {}", user2.email);

    // Call the build_user function to create a user.
    let user3 = build_user(String::from("user@user.com"), String::from("someuser"));
    println!("user3 email: {}", user3.email);

    // Build a user using the function with shorthand assignment.
    let user4 = build_user2(String::from("user2@user2.com"), String::from("user2"));
    println!("user4 email: {}", user4.email);

    // Struct update syntax
    //This uses the values of a previous struct to initialize
    // its fields. 
    // ..user1 will fill out the rest of the struct fields.
    let user5 = User {
        email: String::from("email@email.com"),
        username: String::from("someusername32"),
        ..user1
    };
    println!("{}", user5.active);

    // Tuple Structs
    // Uses the struct keyword but doesn't need named fields.
    // The benefit of this is the tuple struct can be a less
    // verbose struct.
    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Colour(0,0,0);
    let origin = Point(0,0,0);
}

// Function to return a User. 
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Function using shorthand field assignment. 
// The parameter names MUST be the same as the
// field names in the struct. 
//
// Shorthand assignment is done simply as: 
// email,
// username,
// Meaning assign the fields with the same parameter names.
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}