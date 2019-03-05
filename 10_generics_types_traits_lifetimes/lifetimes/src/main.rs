fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("{}", result);

    // Will not compile:
    // x does not live long enough for r to retain its assignment.
    // let r;
    // {
    // let x = 5;
    // r = &x;
    // }
    //
    // println!("r: {}", r);

    // Example 1.
    // The scoping respects the lifetimes of function "longest".
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Example 2.
    // Scoping does not respect the lifetimes of function "longest".
    // string 2 does not live long enough, we know this because in the "longest" function we have
    // both annotated string and string2 to use the same lifetime 'a.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // Lifetime Annotations in Structs.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // We create a struct ImportantExcerpt which holds a refernce to a string which is owned by
    // "let novel".
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/// We need to declare a lifetime parameter.
/// The main aim of lifetimes is to prevent dangling references.
/// In the below example, without the lifetime annotation " &'a str ",
/// we don't know the lifetimes of x,y and we we are not changing ownership, meaning when we return
/// the pointer, it will be dropped as soon as scope is left.
/// Using the annotation.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// By declaring the lifetime annotation at the struct level, this means the struct will not
/// outlive its field, "part" as part is also a pointer.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
